pub const DEFAULT_DIM: (usize, usize) = (0, 5);

pub const DEFAULT_WIDTH: u32 = get_default_size().0;
pub const DEFAULT_HEIGHT: u32 = get_default_size().1;

pub const MAX_SIZE: u32 = find_max_size();

pub const IMAGE_SIZES: &[(&str, &[(u32, u32)])] = &[
    // Square (1:1 Aspect Ratio)
    (
        "1:1",
        &[
            (16, 16),
            (100, 100),
            (500, 500),
            (720, 720),
            (1080, 1080),
            (1200, 1200),
            (2048, 2048),
        ],
    ),
    // Wide (16:9 Aspect Ratio)
    (
        "16:9",
        &[
            (640, 360),
            (854, 480),   // SD 480p
            (1280, 720),  // HD 720p
            (1920, 1080), // Full HD 1080p
            (2560, 1440), // Quad HD
        ],
    ),
    // Ultra-Wide (21:9 Aspect Ratio)
    ("21:9", &[(1280, 540), (2560, 1080), (3440, 1440)]),
    // Portrait (9:16 Aspect Ratio)
    (
        "9:16",
        &[(360, 640), (720, 1280), (1080, 1920), (1440, 2560)],
    ),
    // Traditional (4:3 Aspect Ratio)
    (
        "4:3",
        &[(640, 480), (1024, 768), (1600, 1200), (2048, 1536)],
    ),
    // Photography Standard (3:2 Aspect Ratio)
    ("3:2", &[(600, 400), (1200, 800), (1800, 1200)]),
    // Legacy Square Monitors (5:4 Aspect Ratio)
    ("5:4", &[(800, 640), (1280, 1024)]),
    // Social Media Landscape (1.91:1 Aspect Ratio)
    ("1.91:1", &[(600, 314), (1200, 628)]),
];

const fn get_default_size() -> (u32, u32) {
    IMAGE_SIZES[DEFAULT_DIM.0].1[DEFAULT_DIM.1]
}

const fn find_max_size() -> u32 {
    let mut max_size = 0;
    let mut i = 0;
    while i < IMAGE_SIZES.len() {
        let sizes = IMAGE_SIZES[i].1;
        let mut j = 0;
        while j < sizes.len() {
            let (width, height) = sizes[j];
            let size = width * height;
            if size > max_size {
                max_size = size;
            }
            j += 1;
        }
        i += 1;
    }
    max_size
}
