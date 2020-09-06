use rustc_errors::struct_span_err;
use rustc_hir as hir;
use rustc_hir::def_id::{CrateNum, DefId, LOCAL_CRATE};
use rustc_hir::itemlikevisit::ItemLikeVisitor;
use rustc_middle::ty::TyCtxt;
use rustc_trait_selection::traits::{self, SkipLeakCheck};
use rustc_data_structures::fx::FxHashMap;

pub fn crate_inherent_impls_overlap_check(tcx: TyCtxt<'_>, crate_num: CrateNum) {
    assert_eq!(crate_num, LOCAL_CRATE);
    let krate = tcx.hir().krate();
    krate.visit_all_item_likes(&mut InherentOverlapChecker { tcx });
}

struct InherentOverlapChecker<'tcx> {
    tcx: TyCtxt<'tcx>,
}

impl InherentOverlapChecker<'tcx> {
    /// Checks whether any associated items in impls 1 and 2 share the same identifier and
    /// namespace.
    fn impls_have_common_items(&self, impl1: DefId, impl2: DefId) -> bool {
        let impl_items1 = self.tcx.associated_items(impl1);
        let impl_items2 = self.tcx.associated_items(impl2);

        for item1 in impl_items1.in_definition_order() {
            let collision = impl_items2.filter_by_name_unhygienic(item1.ident.name).any(|item2| {
                // Symbols and namespace match, compare hygienically.
                item1.kind.namespace() == item2.kind.namespace()
                    && item1.ident.normalize_to_macros_2_0()
                        == item2.ident.normalize_to_macros_2_0()
            });

            if collision {
                return true;
            }
        }

        false
    }

    fn check_for_common_items_in_impls(
        &self,
        impl1: DefId,
        impl2: DefId,
        overlap: traits::OverlapResult<'_>,
    ) {
        let impl_items1 = self.tcx.associated_items(impl1);
        let impl_items2 = self.tcx.associated_items(impl2);

        for item1 in impl_items1.in_definition_order() {
            let collision = impl_items2.filter_by_name_unhygienic(item1.ident.name).find(|item2| {
                // Symbols and namespace match, compare hygienically.
                item1.kind.namespace() == item2.kind.namespace()
                    && item1.ident.normalize_to_macros_2_0()
                        == item2.ident.normalize_to_macros_2_0()
            });

            if let Some(item2) = collision {
                let name = item1.ident.normalize_to_macros_2_0();
                let mut err = struct_span_err!(
                    self.tcx.sess,
                    self.tcx.span_of_impl(item1.def_id).unwrap(),
                    E0592,
                    "duplicate definitions with name `{}`",
                    name
                );
                err.span_label(
                    self.tcx.span_of_impl(item1.def_id).unwrap(),
                    format!("duplicate definitions for `{}`", name),
                );
                err.span_label(
                    self.tcx.span_of_impl(item2.def_id).unwrap(),
                    format!("other definition for `{}`", name),
                );

                for cause in &overlap.intercrate_ambiguity_causes {
                    cause.add_intercrate_ambiguity_hint(&mut err);
                }

                if overlap.involves_placeholder {
                    traits::add_placeholder_note(&mut err);
                }

                err.emit();
            }
        }
    }

    fn check_for_overlapping_inherent_impls(&self, impl1_def_id: DefId, impl2_def_id: DefId) {
        traits::overlapping_impls(
            self.tcx,
            impl1_def_id,
            impl2_def_id,
            // We go ahead and just skip the leak check for
            // inherent impls without warning.
            SkipLeakCheck::Yes,
            |overlap| {
                self.check_for_common_items_in_impls(impl1_def_id, impl2_def_id, overlap);
                false
            },
            || true,
        );
    }
}

impl ItemLikeVisitor<'v> for InherentOverlapChecker<'tcx> {
    fn visit_item(&mut self, item: &'v hir::Item<'v>) {
        match &item.kind {
            hir::ItemKind::Enum(e, _) => {
                check_common_overlapping(self, item);
                check_variant_overlapping(self, e, item.hir_id);
            }

            hir::ItemKind::Struct(..)
            | hir::ItemKind::Trait(..)
            | hir::ItemKind::Union(..) => check_common_overlapping(self, item),
            _ => {}
        }
    }

    fn visit_trait_item(&mut self, _trait_item: &hir::TraitItem<'v>) {}

    fn visit_impl_item(&mut self, _impl_item: &hir::ImplItem<'v>) {}
}

fn check_common_overlapping<'v, 'tcx>(
    checker: &mut InherentOverlapChecker<'tcx>,
    item: &'v hir::Item<'v>,
) {
    let ty_def_id = checker.tcx.hir().local_def_id(item.hir_id);
    let impls = checker.tcx.inherent_impls(ty_def_id);

    for (i, &impl1_def_id) in impls.iter().enumerate() {
        for &impl2_def_id in &impls[(i + 1)..] {
            if checker.impls_have_common_items(impl1_def_id, impl2_def_id) {
                checker.check_for_overlapping_inherent_impls(impl1_def_id, impl2_def_id);
            }
        }
    }
}

fn check_variant_overlapping<'v, 'tcx>(
    checker: &mut InherentOverlapChecker<'tcx>,
    en: &hir::EnumDef<'v>,
    item_id: hir::HirId,
) {
    let ty_def_id = checker.tcx.hir().local_def_id(item_id);
    let items = checker.tcx.inherent_impls(ty_def_id).iter().flat_map(|impl_block_id| {
        checker.tcx.associated_items(*impl_block_id).in_definition_order()
    });

    let variant_names: FxHashMap<_, _> = en.variants.iter().map(|v| (&v.ident, v.span)).collect();

    for item in items {
        let item_name = &item.ident;

        if let Some(variant_span) = variant_names.get(&item_name) {
            let name = item.ident.normalize_to_macros_2_0();
            let mut err = struct_span_err!(
                checker.tcx.sess,
                checker.tcx.span_of_impl(item.def_id).unwrap(),
                E0592,
                "duplicate definition with name `{}`",
                name,
            );
            err.span_label(
                *variant_span,
                format!("definition of variant `{}`", name),
            );
            err.span_label(
                checker.tcx.span_of_impl(item.def_id).unwrap(),
                format!("other definition of `{}`", name),
            );
            err.emit();
        }
    }
}
