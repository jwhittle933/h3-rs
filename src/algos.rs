use crate::coordijk::Direction::{self, *};

pub const MAX_ONE_RING_SIZE: u8 = 7;
pub const POLYGON_TO_CELLS_BUFFER: u8 = 12;

pub const DIRECTIONS: [Direction; 6] = [
    JAxesDigit,
    JKAxesDigit,
    KAxesDigit,
    IKAxesDigit,
    IAxesDigit,
    IJAxesDigit,
];

pub const NEXT_RING_DIRECTION: Direction = IAxesDigit;

/// New digit when traversing along class II grids.
pub const NEW_DIGIT_II: [[Direction; 7]; 7] = [
    [
        CenterDigit,
        KAxesDigit,
        JAxesDigit,
        JKAxesDigit,
        IAxesDigit,
        IKAxesDigit,
        IJAxesDigit,
    ],
    [
        KAxesDigit,
        IAxesDigit,
        JKAxesDigit,
        IJAxesDigit,
        IKAxesDigit,
        JAxesDigit,
        CenterDigit,
    ],
    [
        JAxesDigit,
        JKAxesDigit,
        KAxesDigit,
        IAxesDigit,
        IJAxesDigit,
        CenterDigit,
        IKAxesDigit,
    ],
    [
        JKAxesDigit,
        IJAxesDigit,
        IAxesDigit,
        IKAxesDigit,
        CenterDigit,
        KAxesDigit,
        JAxesDigit,
    ],
    [
        IAxesDigit,
        IKAxesDigit,
        IJAxesDigit,
        CenterDigit,
        JAxesDigit,
        JKAxesDigit,
        KAxesDigit,
    ],
    [
        IKAxesDigit,
        JAxesDigit,
        CenterDigit,
        KAxesDigit,
        JKAxesDigit,
        IJAxesDigit,
        IAxesDigit,
    ],
    [
        IJAxesDigit,
        CenterDigit,
        IKAxesDigit,
        JAxesDigit,
        KAxesDigit,
        IAxesDigit,
        JKAxesDigit,
    ],
];

/// New traversal direction when traversing along class II grids.
pub const NEW_ADJUSTMENT_II: [[Direction; 7]; 7] = [
    [
        CenterDigit,
        CenterDigit,
        CenterDigit,
        CenterDigit,
        CenterDigit,
        CenterDigit,
        CenterDigit,
    ],
    [
        CenterDigit,
        KAxesDigit,
        CenterDigit,
        KAxesDigit,
        CenterDigit,
        IKAxesDigit,
        CenterDigit,
    ],
    [
        CenterDigit,
        CenterDigit,
        JAxesDigit,
        JKAxesDigit,
        CenterDigit,
        CenterDigit,
        JAxesDigit,
    ],
    [
        CenterDigit,
        KAxesDigit,
        JKAxesDigit,
        JKAxesDigit,
        CenterDigit,
        CenterDigit,
        CenterDigit,
    ],
    [
        CenterDigit,
        CenterDigit,
        CenterDigit,
        CenterDigit,
        IAxesDigit,
        IAxesDigit,
        IJAxesDigit,
    ],
    [
        CenterDigit,
        IKAxesDigit,
        CenterDigit,
        CenterDigit,
        IAxesDigit,
        IKAxesDigit,
        CenterDigit,
    ],
    [
        CenterDigit,
        CenterDigit,
        JAxesDigit,
        CenterDigit,
        IJAxesDigit,
        CenterDigit,
        IJAxesDigit,
    ],
];

/// New digit when traversing along class III grids.
pub const NEW_DIGIT_III: [[Direction; 7]; 7] = [
    [
        CenterDigit,
        KAxesDigit,
        JAxesDigit,
        JKAxesDigit,
        IAxesDigit,
        IKAxesDigit,
        IJAxesDigit,
    ],
    [
        KAxesDigit,
        JAxesDigit,
        JKAxesDigit,
        IAxesDigit,
        IKAxesDigit,
        IJAxesDigit,
        CenterDigit,
    ],
    [
        JAxesDigit,
        JKAxesDigit,
        IAxesDigit,
        IKAxesDigit,
        IJAxesDigit,
        CenterDigit,
        KAxesDigit,
    ],
    [
        JKAxesDigit,
        IAxesDigit,
        IKAxesDigit,
        IJAxesDigit,
        CenterDigit,
        KAxesDigit,
        JAxesDigit,
    ],
    [
        IAxesDigit,
        IKAxesDigit,
        IJAxesDigit,
        CenterDigit,
        KAxesDigit,
        JAxesDigit,
        JKAxesDigit,
    ],
    [
        IKAxesDigit,
        IJAxesDigit,
        CenterDigit,
        KAxesDigit,
        JAxesDigit,
        JKAxesDigit,
        IAxesDigit,
    ],
    [
        IJAxesDigit,
        CenterDigit,
        KAxesDigit,
        JAxesDigit,
        JKAxesDigit,
        IAxesDigit,
        IKAxesDigit,
    ],
];

/// New traversal direction when traversing along class III grids.
pub const NEW_ADJUSTMENT_III: [[Direction; 7]; 7] = [
    [
        CenterDigit,
        CenterDigit,
        CenterDigit,
        CenterDigit,
        CenterDigit,
        CenterDigit,
        CenterDigit,
    ],
    [
        CenterDigit,
        KAxesDigit,
        CenterDigit,
        JKAxesDigit,
        CenterDigit,
        KAxesDigit,
        CenterDigit,
    ],
    [
        CenterDigit,
        CenterDigit,
        JAxesDigit,
        JAxesDigit,
        CenterDigit,
        CenterDigit,
        IJAxesDigit,
    ],
    [
        CenterDigit,
        JKAxesDigit,
        JAxesDigit,
        JKAxesDigit,
        CenterDigit,
        CenterDigit,
        CenterDigit,
    ],
    [
        CenterDigit,
        CenterDigit,
        CenterDigit,
        CenterDigit,
        IAxesDigit,
        IKAxesDigit,
        IAxesDigit,
    ],
    [
        CenterDigit,
        KAxesDigit,
        CenterDigit,
        CenterDigit,
        IKAxesDigit,
        IKAxesDigit,
        CenterDigit,
    ],
    [
        CenterDigit,
        CenterDigit,
        IJAxesDigit,
        CenterDigit,
        IAxesDigit,
        CenterDigit,
        IJAxesDigit,
    ],
];