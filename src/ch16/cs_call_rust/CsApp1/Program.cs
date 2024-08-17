using System.Runtime.InteropServices;

[DllImport("rust_lib.dll")]
static extern Int32 add_numbers(Int32 number1, Int32 number2);

var addedNumbers = add_numbers(10, 5);
Console.WriteLine(addedNumbers);
Console.ReadLine();
