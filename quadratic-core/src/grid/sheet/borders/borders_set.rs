use crate::{controller::operations::operation::Operation, grid::SheetId};

use super::*;

impl Borders {
    fn set_borders_item(
        item: &BordersUpdatesType,
        border_type: &mut BordersType,
    ) -> BordersUpdatesType {
        item.as_ref()
            .map(|value| border_type.set_from(&value.map_ref(|value| value.map(Into::into))))
            .map(|value| value.map_ref(|value| value.map(Into::into)))
    }

    /// Sets the borders for a selection.
    pub fn set_borders_a1(
        &mut self,
        sheet_id: SheetId,
        borders: &BordersUpdates,
    ) -> Vec<Operation> {
        let reverse_borders = BordersUpdates {
            left: Self::set_borders_item(&borders.left, &mut self.left),
            right: Self::set_borders_item(&borders.right, &mut self.right),
            top: Self::set_borders_item(&borders.top, &mut self.top),
            bottom: Self::set_borders_item(&borders.bottom, &mut self.bottom),
        };

        vec![Operation::SetBordersA1 {
            sheet_id,
            borders: reverse_borders,
        }]
    }

    /// Applies the updates to the borders and returns an update to undo the changes.
    pub fn apply_updates(&mut self, updates: &BordersUpdates) -> BordersUpdates {
        BordersUpdates {
            left: Self::set_borders_item(&updates.left, &mut self.left),
            right: Self::set_borders_item(&updates.right, &mut self.right),
            top: Self::set_borders_item(&updates.top, &mut self.top),
            bottom: Self::set_borders_item(&updates.bottom, &mut self.bottom),
        }
    }
}
