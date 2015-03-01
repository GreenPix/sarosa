
use std::num::Float;
use super::dim::Dimensions;
use super::dim::DimFlags;


pub struct LayoutBuffer(Vec<LayoutBox>);

impl LayoutBuffer {
    pub fn new(size: usize) -> LayoutBuffer {
        LayoutBuffer(Vec::with_capacity(size))
    }
}

// The layout box kids are unsorted (defined by the markup)
// except the one declared with absolute positioning. They will
// end up at the end sorted by z-index.
struct LayoutBox {
    dim: Dimensions,
    // Stores auto/fixed behaviors
    flags: DimFlags,
    kids: usize,
}

macro_rules! layout_for{
    ($child:ident in ($this:ident, $iter:ident) $code:block) => {{
        let mut has_child = $this.kids;

        while has_child > 0 {
            let $child = $iter.next().unwrap();
            has_child -= 1;
            $code
        }
    }};

    ($child:ident in [$nb:ident, $iter:ident] $code:block) => {{
        let mut has_child = $nb;

        while has_child > 0 {
            let $child = $iter.next().unwrap();
            has_child -= 1;
            $code
        }
    }};
}

impl LayoutBox {

    //
    // TODO: FIXME Text nodes must have their size precomputed
    //
    // PRECONDITONS: Everything initialized using PropertyName.
    //
    pub fn compute_layout<'a, I>(
        &mut self,
        iter: &mut I,
        max_width: f32,
        max_height: f32)
            where I: Iterator<Item=&'a mut LayoutBox> + Clone
    {

        // Syntax sugar
        let ref mut d = self.dim;
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
        let max_line_width  = max_width.min(LayoutBox::get_bigger_line_size(self.kids, &mut iter.clone(), max_width));

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

        layout_for!(child in (self, iter) {

            let child_full_width = child.get_max_width(&mut iter.clone());
            let child_is_auto = child.flags.is_auto();

            // Line return ?
            if child_full_width + current_line_width > child_max_width {
                line_return!(stack, current_line_height, current_line_width, current_height_left, d);
            }

            child.dim.content.x = x;
            child.dim.content.y = y;

            child.compute_layout(iter, child_max_width - current_line_width,
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
        });


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

    fn get_bigger_line_size<'a, I>(nb_childs: usize, iter: &mut I, max_width: f32) -> f32
        where I: Iterator<Item=&'a mut LayoutBox>
    {

        let mut max = 0f32;
        let mut current_line_width  = 0f32;

        macro_rules! line_return {
            ($max:ident, $current_line_width:ident) => ({
                $max = $max.max($current_line_width);
                $current_line_width  = 0f32;
            });
        }

        layout_for!(child in [nb_childs, iter] {
            let child_full_width = child.get_max_width(iter);

            // Line return ?
            if child_full_width + current_line_width > max_width {
                line_return!(max, current_line_width);
            }

            current_line_width += child_full_width;

            if child.flags.is_auto() {
                line_return!(max, current_line_width);
            }
        });

        max
    }

    fn get_max_width<'a, I>(&self, iter: &mut I) -> f32
        where I: Iterator<Item=&'a mut LayoutBox>
    {

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
        layout_for!(child in (self, iter) {
            sum += child.get_max_width(iter);

            if sum > max {
                max = sum;
            }
            if child.flags.is_auto() {
                sum = 0f32;
            }
        });


        sum + o
    }
}
