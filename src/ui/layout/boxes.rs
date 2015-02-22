
use std::num::Float;

/// Dimensions for the box model.
///
/// This code follows the css box model
/// in naming and conventions. (all sizes are in pixels)
pub struct Dimensions {
    // Position of the content area relative to the viewport origin
    content: Rect,
    padding: EdgeSizes,
    border: EdgeSizes,
    margin: EdgeSizes,
}

struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

struct EdgeSizes {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

pub struct LayoutBox {
    dim: Dimensions,
    // Store auto/fixed behaviors
    flags: DimFlags,
    kids: Vec<LayoutBox>,
}

bitflags! {
    flags DimFlags: u32 {
        // A text node is WIDTH_FIXED,
        // A node with a style fixed width is naturally WIDTH_FIXED
        const WIDTH_FIXED           = 0b00100000,
        const MARGIN_BOTTTOM_AUTO   = 0b00010000,
        const MARGIN_TOP_AUTO       = 0b00001000,
        const MARGIN_RIGHT_AUTO     = 0b00000100,
        const MARGIN_LEFT_AUTO      = 0b00000010,
        const WIDTH_AUTO            = 0b00000001,
        const MARGIN_X_AUTO         = MARGIN_LEFT_AUTO.bits
                                    | MARGIN_RIGHT_AUTO.bits,
        const MARGIN_Y_AUTO         = MARGIN_TOP_AUTO.bits
                                    | MARGIN_BOTTTOM_AUTO.bits,
    }
}

impl LayoutBox {

    //
    // TODO: FIXME Text nodes must have their size precomputed
    //
    // Par défaut on préfère l'affichage ligne a celui en colonne
    // PRECONDITONS: Everything initialized using PropertyName.
    //
    pub fn compute_layout(&mut self, max_width: f32, max_height: f32) {

        // Syntax sugar
        let ref mut d = self.dim;
        let ref mut kids = self.kids;
        let ref node = self.flags;

        // At this point we don't know d.height / d.width
        // positions
        let mut x = d.content.x + d.padding.left + d.border.left;
        let mut y = d.content.y + d.padding.top  + d.border.top;

        let child_max_width  = max_width
            - d.padding.right - d.padding.left
            - d.border.right  - d.border.left
            - d.margin.right  - d.margin.left;

        let child_max_height = max_height
            - d.padding.bottom - d.padding.top
            - d.border.bottom  - d.border.top
            - d.margin.bottom  - d.margin.top;


        // We can compute directly the d.width
        // and the space for each line
        let max_line_width  = max_width.min(LayoutBox::get_bigger_line_size(kids.iter(), max_width));

        d.content.width = max_line_width;

        // Current line width allow to track the layout progress
        // in the x direction while height allow to track the y direction
        let mut current_line_width  = 0f32;
        let mut current_line_height = 0f32;
        let mut current_height_left = child_max_height;

        // Used for margin (top/bottom)
        let mut stack: Vec<&mut LayoutBox> = Vec::with_capacity(4);

        macro_rules! line_return {

            ($stack:ident,
             $current_line_height:ident,
             $current_line_width:ident,
             $current_height_left:ident,
             $d:ident) => ({

                while let Some(ref mut c) = $stack.pop() {

                    let s = $current_line_height - c.dim.content.height
                        - c.dim.padding.right - c.dim.padding.left
                        - c.dim.border.right  - c.dim.border.left
                        - c.dim.margin.right  - c.dim.margin.left;
                    // We compute the margin top / bottom
                    match (c.flags.has_margin_top_auto(), c.flags.has_margin_bottom_auto()) {
                        (true, true) => {
                            c.dim.margin.top    = s / 2f32;
                            c.dim.margin.bottom = s / 2f32;
                        }
                        (true, false) => {
                            c.dim.margin.left   = s;
                        }
                        (false, true) => {
                            c.dim.margin.right  = s;
                        }
                        _ => ()
                    }
                }

                x = $d.content.x + $d.padding.left + $d.border.left;
                y += $current_line_height;
                $current_height_left -= $current_line_height;
                $current_line_width   = 0f32;
                $current_line_height  = 0f32;
            });
        }

        for child in kids.iter_mut() {

            let child_full_width = child.get_max_width();
            let child_is_auto = child.flags.is_auto();

            // Line return ?
            if child_full_width + current_line_width > child_max_width {
                line_return!(stack, current_line_height, current_line_width, current_height_left, d);
            }

            child.dim.content.x = x;
            child.dim.content.y = y;

            child.compute_layout(child_max_width - current_line_width,
                                 current_height_left);

            current_line_width += child.dim.content.width
                + child.dim.margin.left
                + child.dim.margin.right
                + child.dim.border.left
                + child.dim.border.right;

            // Note: at this point child.margin (top, right) are either fixed
            // or zero (if they were auto). They will be computed in a later pass.
            current_line_height = current_line_height.max(child.dim.content.height
                + child.dim.margin.top
                + child.dim.margin.bottom
                + child.dim.border.top
                + child.dim.border.bottom
            );

            if child.flags.has_margin_top_or_bot_auto() {
                stack.push(child);
            }

            if child_is_auto {
                line_return!(stack, current_line_height, current_line_width, current_height_left, d);
            }
        }


        // Now we do know d.height / d.width
        // We just do some adjustement
        if node.has_width_auto() {
            d.content.width = child_max_width;
        }
        d.content.height = d.content.height.max(child_max_height.min(y + current_line_height));

        // Compute the free space for margin in auto mode:
        let s = child_max_width - d.content.width;

        // We can also compute the margins (left/right) if they're auto:
        match (node.has_margin_right_auto(), node.has_margin_left_auto()) {
            (true, true) => {
                d.margin.left  = s / 2f32;
                d.margin.right = s / 2f32;
            }
            (true, false) => {
                d.margin.left = s;
            }
            (false, true) => {
                d.margin.right = s;
            }
            _ => ()
        }
    }

    fn get_bigger_line_size<'a, I>(kids: I, max_width: f32) -> f32
        where I: Iterator<Item=&'a LayoutBox>
    {

        let mut max = 0f32;
        let mut current_line_width  = 0f32;

        macro_rules! line_return {
            ($max:ident, $current_line_width:ident) => ({
                $max = $max.max($current_line_width);
                $current_line_width  = 0f32;
            });
        }

        for child in kids {
            let child_full_width = child.get_max_width();

            // Line return ?
            if child_full_width + current_line_width > max_width {
                line_return!(max, current_line_width);
            }

            current_line_width += child_full_width;

            if child.flags.is_auto() {
                line_return!(max, current_line_width);
            }
        }

        max
    }

    fn get_max_width(&self) -> f32 {

        let o = self.dim.padding.left
            + self.dim.padding.right
            + self.dim.margin.left
            + self.dim.margin.right
            + self.dim.border.left
            + self.dim.border.right;

        if self.flags.has_width_fixed() {
            return self.dim.content.width + o;
        }

        // Compute max width by using the max width length of a line
        let mut max = 0f32;
        let mut sum = 0f32;
        for child in self.kids.iter() {
            sum += child.get_max_width();

            if sum > max {
                max = sum;
            }
            if child.flags.is_auto() {
                sum = 0f32;
            }
        }


        sum + o
    }
}

impl DimFlags {

    #[inline]
    fn is_auto(&self) -> bool {
        self.intersects(WIDTH_AUTO | MARGIN_X_AUTO)
    }

    #[inline]
    fn has_width_auto(&self) -> bool {
        self.contains(WIDTH_AUTO)
    }

    #[inline]
    fn has_width_fixed(&self) -> bool {
        self.contains(WIDTH_FIXED)
    }

    #[inline]
    fn has_margin_top_or_bot_auto(&self) -> bool {
        self.intersects(MARGIN_Y_AUTO)
    }

    #[inline]
    fn has_margin_left_auto(&self) -> bool {
        self.contains(MARGIN_LEFT_AUTO)
    }

    #[inline]
    fn has_margin_right_auto(&self) -> bool {
        self.contains(MARGIN_RIGHT_AUTO)
    }

    #[inline]
    fn has_margin_top_auto(&self) -> bool {
        self.contains(MARGIN_TOP_AUTO)
    }

    #[inline]
    fn has_margin_bottom_auto(&self) -> bool {
        self.contains(MARGIN_BOTTTOM_AUTO)
    }
}
