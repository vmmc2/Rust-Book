fn main(){
    let ordinal_numbers: [&str; 12] = [
        "first", "second", "third", 
        "fourth", "fifth", "sixth", 
        "seventh", "eighth", "ninth", 
        "tenth", "eleventh", "twelfth"
    ];
    let items: [&str; 12] = [
        "Partridge in a Pear Tree", "Two Turtle Doves", "Three French Hens",
        "Calling Birds", "Gold Rings", "Geese a-Laying",
        "Swans a-Swimming", "Maids a-Milking", "Ladies Dancing",
        "Lords a-Leaping", "Pipers Piping", "Drummers Drumming",
    ];

    for i in 0..=11{
        let mut curr_paragraph: String = String::new();
        curr_paragraph.push_str(format!("On the {} day of Christmas my true love sent to me ", ordinal_numbers[i]).as_str());
        
        let mut j = i;
        while j != 0{
            if j == 1 {
                curr_paragraph += format!("{} ", items[j]).as_str();
            }else{
                curr_paragraph += format!("{}, ", items[j]).as_str();
            }
            j -= 1;
        }
        if i == 0{
            curr_paragraph += format!("a {}.", items[0]).as_str();
        }else{
            curr_paragraph += format!("and a {}.", items[0]).as_str();
        }
    
        println!("{}\n", curr_paragraph);
    }

    return;
}