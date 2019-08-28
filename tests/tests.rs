use mobi::{ExtHeader, Header, HeaderData, MobiHeader, PalmDocHeader};
const BOOK: &[u8] = &[
    76, 111, 114, 100, 95, 111, 102, 95, 116, 104, 101, 95, 82, 105, 110, 103, 115, 95, 45, 95, 70,
    101, 108, 108, 111, 119, 115, 104, 105, 112, 95, 0, 0, 0, 0, 0, 77, 120, 0, 27, 77, 120, 0, 27,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 66, 79, 79, 75, 77, 79, 66, 73, 0, 0, 1, 36, 0,
    0, 0, 0, 1, 36, 0, 0, 9, 112, 0, 0, 0, 0, 0, 0, 19, 4, 0, 0, 0, 1, 0, 0, 25, 99, 0, 0, 0, 2, 0,
    0, 31, 102, 0, 0, 0, 3, 0, 0, 40, 42, 0, 0, 0, 4, 0, 0, 49, 29, 0, 0, 0, 5, 0, 0, 58, 104, 0,
    0, 0, 6, 0, 0, 67, 101, 0, 0, 0, 7, 0, 0, 75, 163, 0, 0, 0, 8, 0, 0, 84, 18, 0, 0, 0, 9, 0, 0,
    93, 2, 0, 0, 0, 10, 0, 0, 101, 249, 0, 0, 0, 11, 0, 0, 110, 237, 0, 0, 0, 12, 0, 0, 120, 49, 0,
    0, 0, 13, 0, 0, 128, 132, 0, 0, 0, 14, 0, 0, 137, 61, 0, 0, 0, 15, 0, 0, 145, 217, 0, 0, 0, 16,
    0, 0, 154, 181, 0, 0, 0, 17, 0, 0, 163, 113, 0, 0, 0, 18, 0, 0, 172, 52, 0, 0, 0, 19, 0, 0,
    180, 219, 0, 0, 0, 20, 0, 0, 189, 233, 0, 0, 0, 21, 0, 0, 198, 237, 0, 0, 0, 22, 0, 0, 207,
    238, 0, 0, 0, 23, 0, 0, 217, 63, 0, 0, 0, 24, 0, 0, 226, 28, 0, 0, 0, 25, 0, 0, 234, 78, 0, 0,
    0, 26, 0, 0, 242, 172, 0, 0, 0, 27, 0, 0, 251, 5, 0, 0, 0, 28, 0, 1, 4, 30, 0, 0, 0, 29, 0, 1,
    13, 3, 0, 0, 0, 30, 0, 1, 21, 84, 0, 0, 0, 31, 0, 1, 30, 77, 0, 0, 0, 32, 0, 1, 39, 38, 0, 0,
    0, 33, 0, 1, 47, 137, 0, 0, 0, 34, 0, 1, 55, 224, 0, 0, 0, 35, 0, 1, 64, 66, 0, 0, 0, 36, 0, 1,
    72, 242, 0, 0, 0, 37, 0, 1, 81, 82, 0, 0, 0, 38, 0, 1, 89, 203, 0, 0, 0, 39, 0, 1, 98, 87, 0,
    0, 0, 40, 0, 1, 106, 250, 0, 0, 0, 41, 0, 1, 115, 109, 0, 0, 0, 42, 0, 1, 124, 57, 0, 0, 0, 43,
    0, 1, 132, 225, 0, 0, 0, 44, 0, 1, 141, 128, 0, 0, 0, 45, 0, 1, 149, 251, 0, 0, 0, 46, 0, 1,
    158, 211, 0, 0, 0, 47, 0, 1, 167, 134, 0, 0, 0, 48, 0, 1, 176, 103, 0, 0, 0, 49, 0, 1, 185, 31,
    0, 0, 0, 50, 0, 1, 194, 7, 0, 0, 0, 51, 0, 1, 202, 149, 0, 0, 0, 52, 0, 1, 211, 117, 0, 0, 0,
    53, 0, 1, 219, 220, 0, 0, 0, 54, 0, 1, 228, 123, 0, 0, 0, 55, 0, 1, 236, 180, 0, 0, 0, 56, 0,
    1, 244, 130, 0, 0, 0, 57, 0, 1, 252, 127, 0, 0, 0, 58, 0, 2, 4, 150, 0, 0, 0, 59, 0, 2, 13,
    102, 0, 0, 0, 60, 0, 2, 21, 193, 0, 0, 0, 61, 0, 2, 30, 89, 0, 0, 0, 62, 0, 2, 38, 106, 0, 0,
    0, 63, 0, 2, 46, 125, 0, 0, 0, 64, 0, 2, 55, 11, 0, 0, 0, 65, 0, 2, 63, 157, 0, 0, 0, 66, 0, 2,
    72, 152, 0, 0, 0, 67, 0, 2, 81, 89, 0, 0, 0, 68, 0, 2, 89, 152, 0, 0, 0, 69, 0, 2, 97, 231, 0,
    0, 0, 70, 0, 2, 106, 168, 0, 0, 0, 71, 0, 2, 115, 21, 0, 0, 0, 72, 0, 2, 123, 175, 0, 0, 0, 73,
    0, 2, 132, 30, 0, 0, 0, 74, 0, 2, 140, 201, 0, 0, 0, 75, 0, 2, 149, 123, 0, 0, 0, 76, 0, 2,
    157, 248, 0, 0, 0, 77, 0, 2, 166, 123, 0, 0, 0, 78, 0, 2, 175, 49, 0, 0, 0, 79, 0, 2, 183, 251,
    0, 0, 0, 80, 0, 2, 192, 104, 0, 0, 0, 81, 0, 2, 200, 206, 0, 0, 0, 82, 0, 2, 209, 68, 0, 0, 0,
    83, 0, 2, 218, 6, 0, 0, 0, 84, 0, 2, 226, 121, 0, 0, 0, 85, 0, 2, 234, 210, 0, 0, 0, 86, 0, 2,
    243, 17, 0, 0, 0, 87, 0, 2, 251, 32, 0, 0, 0, 88, 0, 3, 3, 130, 0, 0, 0, 89, 0, 3, 12, 0, 0, 0,
    0, 90, 0, 3, 20, 145, 0, 0, 0, 91, 0, 3, 29, 80, 0, 0, 0, 92, 0, 3, 38, 16, 0, 0, 0, 93, 0, 3,
    46, 104, 0, 0, 0, 94, 0, 3, 55, 8, 0, 0, 0, 95, 0, 3, 63, 203, 0, 0, 0, 96, 0, 3, 72, 96, 0, 0,
    0, 97, 0, 3, 81, 7, 0, 0, 0, 98, 0, 3, 89, 119, 0, 0, 0, 99, 0, 3, 97, 208, 0, 0, 0, 100, 0, 3,
    106, 49, 0, 0, 0, 101, 0, 3, 114, 167, 0, 0, 0, 102, 0, 3, 123, 67, 0, 0, 0, 103, 0, 3, 132,
    26, 0, 0, 0, 104, 0, 3, 141, 2, 0, 0, 0, 105, 0, 3, 149, 190, 0, 0, 0, 106, 0, 3, 158, 38, 0,
    0, 0, 107, 0, 3, 166, 130, 0, 0, 0, 108, 0, 3, 175, 43, 0, 0, 0, 109, 0, 3, 184, 7, 0, 0, 0,
    110, 0, 3, 192, 155, 0, 0, 0, 111, 0, 3, 201, 156, 0, 0, 0, 112, 0, 3, 209, 57, 0, 0, 0, 113,
    0, 3, 218, 7, 0, 0, 0, 114, 0, 3, 226, 156, 0, 0, 0, 115, 0, 3, 234, 240, 0, 0, 0, 116, 0, 3,
    243, 156, 0, 0, 0, 117, 0, 3, 252, 63, 0, 0, 0, 118, 0, 4, 4, 138, 0, 0, 0, 119, 0, 4, 13, 7,
    0, 0, 0, 120, 0, 4, 21, 112, 0, 0, 0, 121, 0, 4, 29, 244, 0, 0, 0, 122, 0, 4, 38, 130, 0, 0, 0,
    123, 0, 4, 46, 232, 0, 0, 0, 124, 0, 4, 55, 120, 0, 0, 0, 125, 0, 4, 64, 73, 0, 0, 0, 126, 0,
    4, 73, 57, 0, 0, 0, 127, 0, 4, 82, 21, 0, 0, 0, 128, 0, 4, 90, 158, 0, 0, 0, 129, 0, 4, 99, 90,
    0, 0, 0, 130, 0, 4, 107, 230, 0, 0, 0, 131, 0, 4, 116, 119, 0, 0, 0, 132, 0, 4, 124, 213, 0, 0,
    0, 133, 0, 4, 133, 123, 0, 0, 0, 134, 0, 4, 141, 194, 0, 0, 0, 135, 0, 4, 149, 55, 0, 0, 0,
    136, 0, 4, 157, 194, 0, 0, 0, 137, 0, 4, 166, 104, 0, 0, 0, 138, 0, 4, 174, 230, 0, 0, 0, 139,
    0, 4, 183, 217, 0, 0, 0, 140, 0, 4, 192, 40, 0, 0, 0, 141, 0, 4, 200, 228, 0, 0, 0, 142, 0, 4,
    209, 202, 0, 0, 0, 143, 0, 4, 218, 60, 0, 0, 0, 144, 0, 4, 226, 93, 0, 0, 0, 145, 0, 4, 234,
    62, 0, 0, 0, 146, 0, 4, 242, 229, 0, 0, 0, 147, 0, 4, 251, 149, 0, 0, 0, 148, 0, 5, 4, 8, 0, 0,
    0, 149, 0, 5, 12, 81, 0, 0, 0, 150, 0, 5, 21, 32, 0, 0, 0, 151, 0, 5, 29, 107, 0, 0, 0, 152, 0,
    5, 37, 249, 0, 0, 0, 153, 0, 5, 46, 104, 0, 0, 0, 154, 0, 5, 54, 207, 0, 0, 0, 155, 0, 5, 63,
    84, 0, 0, 0, 156, 0, 5, 71, 244, 0, 0, 0, 157, 0, 5, 80, 130, 0, 0, 0, 158, 0, 5, 89, 105, 0,
    0, 0, 159, 0, 5, 97, 247, 0, 0, 0, 160, 0, 5, 106, 81, 0, 0, 0, 161, 0, 5, 114, 51, 0, 0, 0,
    162, 0, 5, 121, 81, 0, 0, 0, 163, 0, 5, 129, 143, 0, 0, 0, 164, 0, 5, 138, 46, 0, 0, 0, 165, 0,
    5, 147, 26, 0, 0, 0, 166, 0, 5, 155, 117, 0, 0, 0, 167, 0, 5, 163, 242, 0, 0, 0, 168, 0, 5,
    172, 142, 0, 0, 0, 169, 0, 5, 181, 38, 0, 0, 0, 170, 0, 5, 189, 142, 0, 0, 0, 171, 0, 5, 198,
    102, 0, 0, 0, 172, 0, 5, 207, 56, 0, 0, 0, 173, 0, 5, 215, 198, 0, 0, 0, 174, 0, 5, 224, 127,
    0, 0, 0, 175, 0, 5, 233, 66, 0, 0, 0, 176, 0, 5, 242, 14, 0, 0, 0, 177, 0, 5, 250, 166, 0, 0,
    0, 178, 0, 6, 2, 230, 0, 0, 0, 179, 0, 6, 11, 124, 0, 0, 0, 180, 0, 6, 20, 43, 0, 0, 0, 181, 0,
    6, 28, 147, 0, 0, 0, 182, 0, 6, 37, 50, 0, 0, 0, 183, 0, 6, 45, 218, 0, 0, 0, 184, 0, 6, 54,
    58, 0, 0, 0, 185, 0, 6, 62, 189, 0, 0, 0, 186, 0, 6, 71, 98, 0, 0, 0, 187, 0, 6, 80, 35, 0, 0,
    0, 188, 0, 6, 88, 156, 0, 0, 0, 189, 0, 6, 97, 102, 0, 0, 0, 190, 0, 6, 105, 225, 0, 0, 0, 191,
    0, 6, 114, 198, 0, 0, 0, 192, 0, 6, 122, 240, 0, 0, 0, 193, 0, 6, 131, 226, 0, 0, 0, 194, 0, 6,
    140, 203, 0, 0, 0, 195, 0, 6, 149, 125, 0, 0, 0, 196, 0, 6, 158, 45, 0, 0, 0, 197, 0, 6, 166,
    231, 0, 0, 0, 198, 0, 6, 175, 76, 0, 0, 0, 199, 0, 6, 184, 52, 0, 0, 0, 200, 0, 6, 192, 223, 0,
    0, 0, 201, 0, 6, 201, 126, 0, 0, 0, 202, 0, 6, 210, 33, 0, 0, 0, 203, 0, 6, 218, 166, 0, 0, 0,
    204, 0, 6, 227, 33, 0, 0, 0, 205, 0, 6, 235, 98, 0, 0, 0, 206, 0, 6, 243, 218, 0, 0, 0, 207, 0,
    6, 252, 107, 0, 0, 0, 208, 0, 7, 5, 2, 0, 0, 0, 209, 0, 7, 13, 158, 0, 0, 0, 210, 0, 7, 22, 12,
    0, 0, 0, 211, 0, 7, 30, 124, 0, 0, 0, 212, 0, 7, 39, 17, 0, 0, 0, 213, 0, 7, 47, 108, 0, 0, 0,
    214, 0, 7, 56, 122, 0, 0, 0, 215, 0, 7, 65, 143, 0, 0, 0, 216, 0, 7, 74, 39, 0, 0, 0, 217, 0,
    7, 82, 228, 0, 0, 0, 218, 0, 7, 90, 235, 0, 0, 0, 219, 0, 7, 99, 190, 0, 0, 0, 220, 0, 7, 108,
    108, 0, 0, 0, 221, 0, 7, 117, 42, 0, 0, 0, 222, 0, 7, 125, 143, 0, 0, 0, 223, 0, 7, 134, 60, 0,
    0, 0, 224, 0, 7, 142, 205, 0, 0, 0, 225, 0, 7, 151, 79, 0, 0, 0, 226, 0, 7, 159, 214, 0, 0, 0,
    227, 0, 7, 168, 9, 0, 0, 0, 228, 0, 7, 176, 103, 0, 0, 0, 229, 0, 7, 185, 8, 0, 0, 0, 230, 0,
    7, 193, 198, 0, 0, 0, 231, 0, 7, 202, 153, 0, 0, 0, 232, 0, 7, 210, 231, 0, 0, 0, 233, 0, 7,
    218, 212, 0, 0, 0, 234, 0, 7, 226, 104, 0, 0, 0, 235, 0, 7, 234, 247, 0, 0, 0, 236, 0, 7, 243,
    138, 0, 0, 0, 237, 0, 7, 252, 19, 0, 0, 0, 238, 0, 8, 4, 198, 0, 0, 0, 239, 0, 8, 13, 85, 0, 0,
    0, 240, 0, 8, 22, 9, 0, 0, 0, 241, 0, 8, 30, 234, 0, 0, 0, 242, 0, 8, 39, 156, 0, 0, 0, 243, 0,
    8, 48, 24, 0, 0, 0, 244, 0, 8, 56, 122, 0, 0, 0, 245, 0, 8, 65, 59, 0, 0, 0, 246, 0, 8, 74, 2,
    0, 0, 0, 247, 0, 8, 82, 42, 0, 0, 0, 248, 0, 8, 90, 155, 0, 0, 0, 249, 0, 8, 99, 11, 0, 0, 0,
    250, 0, 8, 107, 187, 0, 0, 0, 251, 0, 8, 116, 161, 0, 0, 0, 252, 0, 8, 125, 68, 0, 0, 0, 253,
    0, 8, 133, 212, 0, 0, 0, 254, 0, 8, 142, 118, 0, 0, 0, 255, 0, 8, 151, 7, 0, 0, 1, 0, 0, 8,
    159, 151, 0, 0, 1, 1, 0, 8, 168, 99, 0, 0, 1, 2, 0, 8, 176, 243, 0, 0, 1, 3, 0, 8, 185, 104, 0,
    0, 1, 4, 0, 8, 194, 121, 0, 0, 1, 5, 0, 8, 203, 59, 0, 0, 1, 6, 0, 8, 212, 38, 0, 0, 1, 7, 0,
    8, 220, 221, 0, 0, 1, 8, 0, 8, 229, 209, 0, 0, 1, 9, 0, 8, 238, 121, 0, 0, 1, 10, 0, 8, 247,
    17, 0, 0, 1, 11, 0, 8, 255, 240, 0, 0, 1, 12, 0, 9, 8, 127, 0, 0, 1, 13, 0, 9, 17, 56, 0, 0, 1,
    14, 0, 9, 25, 218, 0, 0, 1, 15, 0, 9, 34, 118, 0, 0, 1, 16, 0, 9, 43, 50, 0, 0, 1, 17, 0, 9,
    51, 252, 0, 0, 1, 18, 0, 9, 60, 87, 0, 0, 1, 19, 0, 9, 69, 43, 0, 0, 1, 20, 0, 9, 78, 47, 0, 0,
    1, 21, 0, 9, 86, 243, 0, 0, 1, 22, 0, 9, 95, 116, 0, 0, 1, 23, 0, 9, 103, 239, 0, 0, 1, 24, 0,
    9, 112, 50, 0, 0, 1, 25, 0, 9, 117, 190, 0, 0, 1, 26, 0, 9, 118, 159, 0, 0, 1, 27, 0, 9, 118,
    160, 0, 0, 1, 28, 0, 9, 119, 144, 0, 0, 1, 29, 0, 9, 122, 120, 0, 0, 1, 30, 0, 9, 124, 24, 0,
    0, 1, 31, 0, 10, 20, 91, 0, 0, 1, 32, 0, 10, 81, 174, 0, 0, 1, 33, 0, 10, 81, 210, 0, 0, 1, 34,
    0, 10, 81, 254, 0, 0, 1, 35, 0, 0, 0, 2, 0, 0, 0, 17, 145, 229, 1, 26, 16, 0, 0, 0, 0, 0, 77,
    79, 66, 73, 0, 0, 0, 232, 0, 0, 0, 2, 0, 0, 253, 233, 204, 83, 211, 193, 0, 0, 0, 6, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    0, 0, 1, 28, 0, 0, 5, 80, 0, 0, 0, 42, 0, 0, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 1,
    31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255,
    255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 32, 0, 0, 0, 1,
    0, 0, 1, 34, 0, 0, 0, 1, 0, 0, 1, 33, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255,
    0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 7, 0, 0, 1, 28, 69, 88, 84, 72, 0,
    0, 4, 85, 0, 0, 0, 11, 0, 0, 0, 101, 0, 0, 0, 36, 72, 97, 114, 112, 101, 114, 67, 111, 108,
    108, 105, 110, 115, 32, 80, 117, 98, 108, 105, 115, 104, 101, 114, 115, 32, 76, 116, 100, 0, 0,
    0, 103, 0, 0, 3, 45, 60, 104, 51, 62, 70, 114, 111, 109, 32, 76, 105, 98, 114, 97, 114, 121,
    32, 74, 111, 117, 114, 110, 97, 108, 60, 47, 104, 51, 62, 60, 112, 62, 78, 101, 119, 32, 76,
    105, 110, 101, 32, 67, 105, 110, 101, 109, 97, 32, 119, 105, 108, 108, 32, 98, 101, 32, 114,
    101, 108, 101, 97, 115, 105, 110, 103, 32, 34, 84, 104, 101, 32, 76, 111, 114, 100, 32, 111,
    102, 32, 116, 104, 101, 32, 82, 105, 110, 103, 115, 34, 32, 116, 114, 105, 108, 111, 103, 121,
    32, 105, 110, 32, 116, 104, 114, 101, 101, 32, 115, 101, 112, 97, 114, 97, 116, 101, 32, 105,
    110, 115, 116, 97, 108, 108, 109, 101, 110, 116, 115, 44, 32, 97, 110, 100, 32, 72, 111, 117,
    103, 104, 116, 111, 110, 32, 77, 105, 102, 102, 108, 105, 110, 32, 84, 111, 108, 107, 105, 101,
    110, 39, 115, 32, 85, 46, 83, 46, 32, 112, 117, 98, 108, 105, 115, 104, 101, 114, 32, 115, 105,
    110, 99, 101, 32, 116, 104, 101, 32, 114, 101, 108, 101, 97, 115, 101, 32, 111, 102, 32, 84,
    104, 101, 32, 72, 111, 98, 98, 105, 116, 32, 105, 110, 32, 49, 57, 51, 56, 32, 119, 105, 108,
    108, 32, 98, 101, 32, 114, 101, 45, 114, 101, 108, 101, 97, 115, 105, 110, 103, 32, 101, 97,
    99, 104, 32, 118, 111, 108, 117, 109, 101, 32, 111, 102, 32, 116, 104, 101, 32, 116, 114, 105,
    108, 111, 103, 121, 32, 115, 101, 112, 97, 114, 97, 116, 101, 108, 121, 32, 97, 110, 100, 32,
    105, 110, 32, 97, 32, 98, 111, 120, 101, 100, 32, 115, 101, 116, 32, 40, 73, 83, 66, 78, 32,
    48, 45, 54, 49, 56, 45, 49, 53, 51, 57, 55, 45, 55, 46, 32, 36, 50, 50, 59, 32, 112, 97, 112,
    46, 32, 73, 83, 66, 78, 32, 48, 45, 54, 49, 56, 45, 49, 53, 51, 57, 54, 45, 57, 46, 32, 36, 49,
    50, 41, 46, 32, 60, 98, 114, 32, 47, 62, 67, 111, 112, 121, 114, 105, 103, 104, 116, 32, 50,
    48, 48, 49, 32, 82, 101, 101, 100, 32, 66, 117, 115, 105, 110, 101, 115, 115, 32, 73, 110, 102,
    111, 114, 109, 97, 116, 105, 111, 110, 44, 32, 73, 110, 99, 46, 32, 60, 47, 112, 62, 60, 104,
    51, 62, 82, 101, 118, 105, 101, 119, 60, 47, 104, 51, 62, 60, 112, 62, 39, 65, 110, 32, 101,
    120, 116, 114, 97, 111, 114, 100, 105, 110, 97, 114, 121, 32, 98, 111, 111, 107, 46, 32, 73,
    116, 32, 100, 101, 97, 108, 115, 32, 119, 105, 116, 104, 32, 97, 32, 115, 116, 117, 112, 101,
    110, 100, 111, 117, 115, 32, 116, 104, 101, 109, 101, 46, 32, 73, 116, 32, 108, 101, 97, 100,
    115, 32, 117, 115, 32, 116, 104, 114, 111, 117, 103, 104, 32, 97, 32, 115, 117, 99, 99, 101,
    115, 115, 105, 111, 110, 32, 111, 102, 32, 115, 116, 114, 97, 110, 103, 101, 32, 97, 110, 100,
    32, 97, 115, 116, 111, 110, 105, 115, 104, 105, 110, 103, 32, 101, 112, 105, 115, 111, 100,
    101, 115, 44, 32, 115, 111, 109, 101, 32, 111, 102, 32, 116, 104, 101, 109, 32, 109, 97, 103,
    110, 105, 102, 105, 99, 101, 110, 116, 44, 32, 105, 110, 32, 97, 32, 114, 101, 103, 105, 111,
    110, 32, 119, 104, 101, 114, 101, 32, 101, 118, 101, 114, 121, 116, 104, 105, 110, 103, 32,
    105, 115, 32, 105, 110, 118, 101, 110, 116, 101, 100, 44, 32, 102, 111, 114, 101, 115, 116, 44,
    32, 109, 111, 111, 114, 44, 32, 114, 105, 118, 101, 114, 44, 32, 119, 105, 108, 100, 101, 114,
    110, 101, 115, 115, 44, 32, 116, 111, 119, 110, 32, 97, 110, 100, 32, 116, 104, 101, 32, 114,
    97, 99, 101, 115, 32, 119, 104, 105, 99, 104, 32, 105, 110, 104, 97, 98, 105, 116, 32, 116,
    104, 101, 109, 46, 39, 32, 84, 104, 101, 32, 79, 98, 115, 101, 114, 118, 101, 114, 32, 39, 65,
    109, 111, 110, 103, 32, 116, 104, 101, 32, 103, 114, 101, 97, 116, 101, 115, 116, 32, 119, 111,
    114, 107, 115, 32, 111, 102, 32, 105, 109, 97, 103, 105, 110, 97, 116, 105, 118, 101, 32, 102,
    105, 99, 116, 105, 111, 110, 32, 111, 102, 32, 116, 104, 101, 32, 116, 119, 101, 110, 116, 105,
    101, 116, 104, 32, 99, 101, 110, 116, 117, 114, 121, 46, 39, 32, 83, 117, 110, 100, 97, 121,
    32, 84, 101, 108, 101, 103, 114, 97, 112, 104, 32, 60, 47, 112, 62, 0, 0, 0, 100, 0, 0, 0, 24,
    74, 46, 32, 82, 46, 32, 82, 46, 32, 84, 111, 108, 107, 105, 101, 110, 0, 0, 1, 247, 0, 0, 0,
    50, 76, 111, 114, 100, 32, 111, 102, 32, 116, 104, 101, 32, 82, 105, 110, 103, 115, 32, 45, 32,
    70, 101, 108, 108, 111, 119, 115, 104, 105, 112, 32, 111, 102, 32, 116, 104, 101, 32, 82, 105,
    110, 103, 0, 0, 0, 106, 0, 0, 0, 33, 50, 48, 49, 48, 45, 49, 50, 45, 50, 49, 84, 48, 48, 58,
    48, 48, 58, 48, 48, 43, 48, 48, 58, 48, 48, 0, 0, 0, 108, 0, 0, 0, 51, 99, 97, 108, 105, 98,
    114, 101, 32, 40, 48, 46, 55, 46, 51, 49, 41, 32, 91, 104, 116, 116, 112, 58, 47, 47, 99, 97,
    108, 105, 98, 114, 101, 45, 101, 98, 111, 111, 107, 46, 99, 111, 109, 93, 0, 0, 0, 104, 0, 0,
    0, 21, 57, 55, 56, 48, 50, 54, 49, 49, 48, 50, 51, 49, 54, 0, 0, 0, 106, 0, 0, 0, 33, 50, 48,
    49, 48, 45, 49, 50, 45, 50, 49, 84, 48, 48, 58, 48, 48, 58, 48, 48, 43, 48, 48, 58, 48, 48, 0,
    0, 0, 201, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 203, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 202, 0, 0,
    0, 12, 0, 0, 0, 1, 0, 0, 0, 76, 111, 114, 100, 32, 111, 102, 32, 116, 104, 101, 32, 82, 105,
    110, 103, 115, 32, 45, 32, 70, 101, 108, 108, 111, 119, 115, 104, 105, 112, 32, 111, 102, 32,
    116, 104, 101, 32, 82, 105, 110, 103,
];
#[test]
fn parse_header() {
    let header = Header {
        name: String::from("Lord_of_the_Rings_-_Fellowship_\u{0}"),
        attributes: 0,
        version: 0,
        created: 1299709979,
        modified: 1299709979,
        backup: 0,
        modnum: 0,
        app_info_id: 0,
        sort_info_id: 0,
        typ_e: String::from("BOOK"),
        creator: String::from("MOBI"),
        unique_id_seed: 292,
        next_record_list_id: 0,
        num_of_records: 292,
    };
    let parsed_header = Header::parse(BOOK);
    assert_eq!(header, parsed_header);
}
#[test]
fn parse_palmdocheader() {
    let pdheader = PalmDocHeader {
        compression: 2,
        text_length: 1151461,
        record_count: 282,
        record_size: 4096,
        encryption_type: 0,
    };
    let parsed_header = PalmDocHeader::parse(
        BOOK,
        Header::get_headers_u16(BOOK, HeaderData::NumOfRecords),
    );
    assert_eq!(pdheader, parsed_header);
}
#[test]
fn parse_mobiheader() {
    let mobiheader = MobiHeader {
        identifier: 232,
        header_length: 2,
        mobi_type: 65001,
        text_encoding: 3428045761,
        id: 6,
        gen_version: 4294967295,
        first_non_book_index: 284,
        name: String::from("Lord of the Rings - Fellowship of the Ring"),
        name_offset: 1360,
        name_length: 42,
        language: 2057,
        input_language: 0,
        output_language: 0,
        format_version: 6,
        first_image_index: 287,
        first_huff_record: 0,
        huff_record_count: 0,
        first_data_record: 0,
        data_record_count: 0,
        exth_flags: 80,
        has_exth_header: true,
        drm_offset: 4294967295,
        drm_count: 0,
        drm_size: 0,
        drm_flags: 0,
        last_image_record: 288,
        fcis_record: 290,
        flis_record: 289,
    };
    let parsed_header = MobiHeader::parse(
        BOOK,
        Header::get_headers_u16(BOOK, HeaderData::NumOfRecords),
    );
    assert_eq!(mobiheader, parsed_header);
}
#[test]
fn parse_extheader() {
    let extheader = ExtHeader {
        identifier: 1163416648,
        header_length: 1109,
        record_count: 11,
        records: vec![
            (36, 101, String::from("HarperCollins Publishers Ltd")),
            (813, 103, String::from("<h3>From Library Journal</h3><p>New Line Cinema will be releasing \"The Lord of the Rings\" trilogy in three separate installments, and Houghton Mifflin Tolkien\'s U.S. publisher since the release of The Hobbit in 1938 will be re-releasing each volume of the trilogy separately and in a boxed set (ISBN 0-618-15397-7. $22; pap. ISBN 0-618-15396-9. $12). <br />Copyright 2001 Reed Business Information, Inc. </p><h3>Review</h3><p>\'An extraordinary book. It deals with a stupendous theme. It leads us through a succession of strange and astonishing episodes, some of them magnificent, in a region where everything is invented, forest, moor, river, wilderness, town and the races which inhabit them.\' The Observer \'Among the greatest works of imaginative fiction of the twentieth century.\' Sunday Telegraph </p>")),
            (24, 100, String::from("J. R. R. Tolkien")),
            (50, 503, String::from("Lord of the Rings - Fellowship of the Ring")),
            (33, 106, String::from("2010-12-21T00:00:00+00:00")),
            (51, 108, String::from("calibre (0.7.31) [http://calibre-ebook.com]")),
            (21, 104, String::from("9780261102316")),
            (33, 106, String::from("2010-12-21T00:00:00+00:00")),
            (12, 201, String::from("\u{0}\u{0}\u{0}\u{0}")),
            (12, 203, String::from("\u{0}\u{0}\u{0}\u{0}")),
            (12, 202, String::from("\u{0}\u{0}\u{0}\u{1}")),
        ],
    };
    let parsed_header = ExtHeader::parse(
        BOOK,
        Header::get_headers_u16(BOOK, HeaderData::NumOfRecords),
    );
    assert_eq!(extheader, parsed_header);
}
