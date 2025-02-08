fn main() {
    let var1 = "hai \
    \"rust\" \
    and \
    \"world\"";
    println!("{}", var1);

    // multiline string literal
    let var3 = "baris satu
baris dua
baris tiga";
    println!("{}", var3);

    // raw string
    let var5 = r#"
        {
            "name": "tim drake",
            "gender": "male"
        }
    "#;
    println!("{}", var5);

    let var6 = "
    {
        \"name\": \"cassandra cain\",
        \"gender\": \"female\"
    }
";
println!("{}", var6);
}
