fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 96.6 - 4.9;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Result is 0

    // remainder
    let remainder = 43 % 4;

    /// boolean
    let t = true;

    let f: bool = false;

    /// char
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = 'ð»';
    let kana_a = 'ã¢';

    /// tuple
    let tup: (i32, f64, u8) = (600, 5.2, 3);

    let (x, y, z) = tup; // ãã¿ã¼ã³ãããã³ã°ã«ããåè§£

    let six_hundred = tup.0; // tupã®0çªç®ã«ã¢ã¯ã»ã¹

    let five_point_two = tup.1; // tupã®1çªç®ã«ã¢ã¯ã»ã¹

    let three = tup.2; // tupã®2çªç®ã«ã¢ã¯ã»ã¹

    /// array
    // Rustã®éåã¯åºå®ç³»ãªã®ã§ãä¸åº¦å®£è¨ãããããµã¤ãºãä¼¸ã°ããã¨ãç¸®ãããã¨ãã§ããªã
    let a = [1, 2, 3, 4, 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32: 5] = [1, 2, 3, 4, 5];

    let a = [3; 5]; // åæå¤3ã5åæã£ãéåãåæå

    let first = a[0]; // éåã®1çªç®ã«ã¢ã¯ã»ã¹

    let second = a[1]; // éåã®2çªç®ã«ã¢ã¯ã»ã¹
}
