# RustHour
Hackathon Challenge

ByteCode Challenge : 
I declared a ByteCode of type Vec<String> to store Bytecodes as strings and used vectors as stacks.
then to process each line of bytecode i used enums for opcodes and used vec to store variable names and values separately.
The vec is used as stack and the pop operatrion is done whenever read command is issued and push operation is done whenever load command is issued.

For a given opcode like ADD,SUB,MUl we can perform addition , subtraction and multiplication.

finally the result is evaluated and returned.