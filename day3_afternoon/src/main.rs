// helper functions

fn demonstrate_borrow_checker(){

    println!("As covered in the morning session, Rust's emphasis on both memory safety and user control of that memory takes it down some inetresting, unchartered paths.");
    println!("One of those innovations is Rust's BORROW CHECKER");

    println!("Recall that as we saw before, instead of transferring ownership when calling a function, you can let a function borrow the value.");

    #[derive(Debug)]
    struct Point(i32, i32);
    fn add(p1: &Point, p2: &Point) -> Point {
        Point(p1.0 + p2.0, p1.1 + p2.1)
    }
    let p1 = Point(3, 4);
    let p2 = Point(10, 20);
    let p3 = add(&p1, &p2);
    println!("{p1:?} + {p2:?} = {p3:?}");

    println!("\nTo avoid any messy memory errors, Rust's borrow checker enforces the following rules\n\n");

    println!("\n1. A reference cannot outlive the value it borrows.\n");
    let x_ref = {
        let _x = 10;
        // &x
    };
    // println!("x: {x_ref}");
    println!("The above line would therefore yield an error if run.\n");

    println!("\n2. For a given value, at any time, you can EITHER have one or more shared references to the value OR have exactly one exclusive reference to the value. This is also known as the ALISING rule.\n");

    let a: i32 = 10;
    let b: &i32 = &a;
    {
        // let c: &mut i32 = &mut a;
        // *c = 20;
    }
    println!("a: {a}");
    println!("b: {b}");
    println!("\nFor that same reason, the above line could possibly yield an error.\n");

    println!("There are also further concepts useful to check out, such as interior mutability: https://google.github.io/comprehensive-rust/borrowing/interior-mutability.html");

}

pub struct User {
    name: String,
    age: u32,
    height: f32,
    visit_count: usize,
    last_blood_pressure: Option<(u32, u32)>,
}

pub struct Measurements {
    height: f32,
    blood_pressure: (u32, u32),
}

pub struct HealthReport<'a> {
    patient_name: &'a str,
    visit_count: u32,
    height_change: f32,
    blood_pressure_change: Option<(i32, i32)>,
}

impl User {

    pub fn new(name: String, age: u32, height: f32) -> Self {
        Self { name, age, height, visit_count: 0, last_blood_pressure: None }
    }

    pub fn visit_doctor(&mut self, measurements: Measurements) -> HealthReport {
        self.visit_count += 1;
        let height_change = measurements.height - self.height;
        let blood_pressure_change = if let Some(last_bp) = self.last_blood_pressure {
            let change_systolic = measurements.blood_pressure.0 as i32 - last_bp.0 as i32;
            let change_diastolic = measurements.blood_pressure.1 as i32 - last_bp.1 as i32;
            Some((change_systolic, change_diastolic))
        } else {
            None
        };
        self.last_blood_pressure = Some(measurements.blood_pressure);
        self.height = measurements.height;
        HealthReport {
            patient_name: &self.name,
            visit_count: self.visit_count as u32,
            height_change,
            blood_pressure_change,
        }
    }

}

fn render_lifetimes(){
    println!("Another concept instrinsic to the borrow checker is that of LIFETIMES");
    println!("More information on its complexities can be found here: https://google.github.io/comprehensive-rust/lifetimes.html");
}

enum WireType {
    Varint,
    Len,
}

#[derive(Debug)]
enum FieldValue<'a> {
    Varint(u64),
    Len(&'a [u8]),
}

#[derive(Debug)]
struct Field<'a> {
    field_num: u64,
    value: FieldValue<'a>,
}

trait ProtoMessage<'a>: Default {
    fn add_field(&mut self, field: Field<'a>);
}

impl From<u64> for WireType {
    fn from(value: u64) -> Self {
        match value {
            0 => WireType::Varint,
            2 => WireType::Len,
            _ => panic!("Invalid wire type: {value}"),
        }
    }
}

impl<'a> FieldValue<'a> {
    fn as_str(&self) -> &'a str {
        let FieldValue::Len(data) = self else {
            panic!("Expected string to be a `Len` field");
        };
        std::str::from_utf8(data).expect("Invalid string")
    }

    fn as_bytes(&self) -> &'a [u8] {
        let FieldValue::Len(data) = self else {
            panic!("Expected bytes to be a `Len` field");
        };
        data
    }

    fn as_u64(&self) -> u64 {
        let FieldValue::Varint(value) = self else {
            panic!("Expected `u64` to be a `Varint` field");
        };
        *value
    }
}

fn parse_varint(data: &[u8]) -> (u64, &[u8]) {
    for i in 0..7 {
        let Some(b) = data.get(i) else {
            panic!("Not enough bytes for varint");
        };
        if b & 0x80 == 0 {
            let mut value = 0u64;
            for b in data[..=i].iter().rev() {
                value = (value << 7) | (b & 0x7f) as u64;
            }
            return (value, &data[i + 1..]);
        }
    }
    panic!("Too many bytes for varint");
}

fn unpack_tag(tag: u64) -> (u64, WireType) {
    let field_num = tag >> 3;
    let wire_type = WireType::from(tag & 0x7);
    (field_num, wire_type)
}

fn parse_field(data: &[u8]) -> (Field, &[u8]) {
    let (tag, remainder) = parse_varint(data);
    let (field_num, wire_type) = unpack_tag(tag);
    
    let (field_value, remainder) = match wire_type {
        WireType::Varint => {
            let (value, rem) = parse_varint(remainder);
            (FieldValue::Varint(value), rem)
        }
        WireType::Len => {
            let (length, rem) = parse_varint(remainder);
            let len_bytes = &rem[..length as usize];
            (FieldValue::Len(len_bytes), &rem[length as usize..])
        }
    };

    (Field { field_num, value: field_value }, remainder)
}

fn parse_message<'a, T: ProtoMessage<'a>>(mut data: &'a [u8]) -> T {
    let mut result = T::default();
    while !data.is_empty() {
        let parsed = parse_field(data);
        result.add_field(parsed.0);
        data = parsed.1;
    }
    result
}

#[derive(Debug, Default)]
struct PhoneNumber<'a> {
    number: &'a str,
    type_: &'a str,
}

#[derive(Debug, Default)]
struct Person<'a> {
    name: &'a str,
    id: u64,
    phone: Vec<PhoneNumber<'a>>,
}

impl<'a> ProtoMessage<'a> for PhoneNumber<'a> {
    fn add_field(&mut self, field: Field<'a>) {
        match field.field_num {
            1 => self.number = field.value.as_str(),
            2 => self.type_ = field.value.as_str(),
            _ => panic!("Unknown field number for PhoneNumber"),
        }
    }
}

impl<'a> ProtoMessage<'a> for Person<'a> {
    fn add_field(&mut self, field: Field<'a>) {
        match field.field_num {
            1 => self.name = field.value.as_str(),
            2 => self.id = field.value.as_u64(),
            3 => { 
                let phone_number_data = field.value.as_bytes();
                let phone_number: PhoneNumber = parse_message(phone_number_data);
                self.phone.push(phone_number);
            },
            _ => panic!("Unknown field number for Person"),
        }
    }
}

// main function

fn main() {

    println!("~~~ Comprehensive Rust: Day 3 Afternoon ~~~");

    // borrowing

    demonstrate_borrow_checker();

    // health statistics exercise

    let bob = User::new(String::from("Bob"), 32, 155.2);
    println!("I'm {} and my age is {}", bob.name, bob.age);

    #[test]
    fn test_visit() {
        let mut bob = User::new(String::from("Bob"), 32, 155.2);
        assert_eq!(bob.visit_count, 0);
        let report =
            bob.visit_doctor(Measurements { height: 156.1, blood_pressure: (120, 80) });
        assert_eq!(report.patient_name, "Bob");
        assert_eq!(report.visit_count, 1);
        assert_eq!(report.blood_pressure_change, None);
        assert!((report.height_change - 0.9).abs() < 0.00001);

        let report =
            bob.visit_doctor(Measurements { height: 156.1, blood_pressure: (115, 76) });

        assert_eq!(report.visit_count, 2);
        assert_eq!(report.blood_pressure_change, Some((-5, -4)));
        assert_eq!(report.height_change, 0.0);
    }

    // lifetimes

    render_lifetimes();

    // protobuf parsing

    let person_id: Person = parse_message(&[0x10, 0x2a]);
    assert_eq!(person_id, Person { name: "", id: 42, phone: vec![] });

    let person_name: Person = parse_message(&[
        0x0a, 0x0e, 0x62, 0x65, 0x61, 0x75, 0x74, 0x69, 0x66, 0x75, 0x6c, 0x20,
        0x6e, 0x61, 0x6d, 0x65,
    ]);
    assert_eq!(person_name, Person { name: "beautiful name", id: 0, phone: vec![] });

    let person_name_id: Person =
        parse_message(&[0x0a, 0x04, 0x45, 0x76, 0x61, 0x6e, 0x10, 0x16]);
    assert_eq!(person_name_id, Person { name: "Evan", id: 22, phone: vec![] });

    let phone: Person = parse_message(&[
        0x0a, 0x00, 0x10, 0x00, 0x1a, 0x16, 0x0a, 0x0e, 0x2b, 0x31, 0x32, 0x33,
        0x34, 0x2d, 0x37, 0x37, 0x37, 0x2d, 0x39, 0x30, 0x39, 0x30, 0x12, 0x04,
        0x68, 0x6f, 0x6d, 0x65,
    ]);
    assert_eq!(
        phone,
        Person {
            name: "",
            id: 0,
            phone: vec![PhoneNumber { number: "+1234-777-9090", type_: "home" },],
        }
    );
    let person: Person = parse_message(&[
        0x0a, 0x07, 0x6d, 0x61, 0x78, 0x77, 0x65, 0x6c, 0x6c, 0x10, 0x2a, 0x1a,
        0x16, 0x0a, 0x0e, 0x2b, 0x31, 0x32, 0x30, 0x32, 0x2d, 0x35, 0x35, 0x35,
        0x2d, 0x31, 0x32, 0x31, 0x32, 0x12, 0x04, 0x68, 0x6f, 0x6d, 0x65, 0x1a,
        0x18, 0x0a, 0x0e, 0x2b, 0x31, 0x38, 0x30, 0x30, 0x2d, 0x38, 0x36, 0x37,
        0x2d, 0x35, 0x33, 0x30, 0x38, 0x12, 0x06, 0x6d, 0x6f, 0x62, 0x69, 0x6c,
        0x65,
    ]);
    assert_eq!(
        person,
        Person {
            name: "maxwell",
            id: 42,
            phone: vec![
                PhoneNumber { number: "+1202-555-1212", type_: "home" },
                PhoneNumber { number: "+1800-867-5308", type_: "mobile" },
            ]
        }
    );

}
