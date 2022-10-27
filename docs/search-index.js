var searchIndex = JSON.parse('{\
"claw":{"doc":"Claw is a programming language that compiles into Scratch …","t":[0,0,0,0,0,0,6,6,0,5,5,5,5,0,6,13,13,13,4,13,4,13,13,13,4,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,5,5,5,5,5,5,5,5,3,3,3,3,3,12,11,11,11,11,11,12,12,11,11,11,11,11,11,11,11,11,11,12,12,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,12,12,12,12,12,11,11,12,12,12,12,11,11,11,11,11,12,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,17,17,5,5],"n":["middle","parse","sb3","verify","stdlib","ty","AST","ParseError","ast","build_report","create_parser","get_source","parse","parser","AST","Boolean","Call","Call","Declaration","Declare","Expression","Func","Number","Sprite","Statement","Text","append_to","append_to","append_to","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone_into","clone_into","clone_into","fmt","fmt","fmt","from","from","from","get_iter","get_iter","get_iter","into","into","into","len","len","len","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","body","declarations","events","items","kind","name","name","0","0","0","args","module","name","args","module","name","create_call","create_comment","create_declare","create_expression","create_func","create_parser","create_sprite","create_statement","Block","Costume","Meta","Project","Target","agent","append_to","append_to","append_to","append_to","append_to","asset_id","blocks","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","costumes","data_format","default","default","deserialize","deserialize","deserialize","deserialize","deserialize","extensions","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","into","into","into","into","into","is_stage","len","len","len","len","len","md5ext","meta","monitors","name","name","new_sprite","new_stage","next","opcode","parent","semver","serialize","serialize","serialize","serialize","serialize","sounds","targets","top_level","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","variables","vm","SB3_DEFINITIONS","SB3_SCHEMA","verify","verify_string"],"q":["claw","","","","claw::middle","","claw::parse","","","","","","","","claw::parse::ast","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","claw::parse::ast::Declaration","","","","","","","claw::parse::ast::Expression","","","","","","claw::parse::ast::Statement","","","claw::parse::parser","","","","","","","","claw::sb3","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","claw::verify","","",""],"d":["Re-imported from <code>claw_middle</code>.","Re-imported from <code>claw_parse</code>.","Re-imported from <code>claw_sb3</code>.","","","","Public type used to represent a complete program in AST …","Error type returned by the parsers.","Contains the abstract syntax tree definitions.","Transforms a result to use [<code>ariadne</code>]’s <code>Report</code>.","Creates the main <code>Parser</code> to be used when parsing a Claw …","Returns the <code>Source</code> of a string reference.","Parses a <code>str</code> into <code>AST</code>.","Contains the logic of the actual parser, written using […","Public type used to represent a complete program in AST …","A boolean.","A function call that does not return anything.","A function that returns a value.","Declares constants.","An enum declaration.","An expression to be used from within a <code>Statement</code>.","A function declaration.","A number, whether decimal or not.","A sprite declaration.","A statement that does not return a value.","A string of text.","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","The code that is run when the function is called.","The declarations within the sprite.","The events that trigger the function.","The items declared within the enum.","The kind of enum being declared.","The name of the function.","The name of the sprite.","","","","The arguments passed to the function.","The module the function is from.","The name of the function.","The arguments passed to the function.","The module the function is from.","The name of the function.","Creates a call parser.","Creates a comment parser.","Creates an enum declaration parser.","Creates an expression parser.","Creates a function declaration parser.","Creates the main <code>Parser</code> to be used when parsing a Claw …","Creates a sprite declaration parser.","Creates a statement parser.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Shared definitions between multiple schemas, though right …","The schema that a <code>project.json</code> file is checked against.","The main function that verifies a [<code>serde_json::Value</code>] is a …","Conveniance function that verifies the string form of a …"],"i":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,15,14,15,0,1,0,1,15,1,0,15,1,14,15,1,14,15,1,14,15,1,14,15,1,14,15,1,14,15,1,14,15,1,14,15,1,14,15,1,14,15,1,14,15,1,14,15,1,14,15,1,14,15,26,27,26,28,28,26,27,29,30,31,32,32,32,33,33,33,0,0,0,0,0,0,0,0,0,0,0,0,0,20,21,22,19,20,23,22,23,21,22,19,20,23,21,22,19,20,23,23,22,19,20,21,22,19,20,23,19,21,22,19,20,23,21,22,19,20,23,21,22,19,20,23,23,21,22,19,20,23,22,19,19,22,23,23,23,21,21,21,20,21,22,19,20,23,23,19,21,21,22,19,20,23,21,22,19,20,23,21,22,19,20,23,23,20,0,0,0,0],"f":[0,0,0,0,0,0,0,0,0,[[[8,[[3,[1,2]],[3,[[7,[4,[6,[5]]]],2]]]]],[[8,[[3,[1,2]],[10,[[9,[[6,[5]]]],2]]]]]],[[],[[11,[4,[3,[1,2]]]]]],[[],12],[13,[[8,[[3,[1,2]],[10,[[9,[[6,[5]]]],2]]]]]],0,0,0,0,0,0,0,0,0,0,0,0,0,[3],[3],[3],[[]],[[]],[[]],[[]],[[]],[[]],[1,1],[14,14],[15,15],[[]],[[]],[[]],[[1,16],[[8,[17]]]],[[14,16],[[8,[17]]]],[[15,16],[[8,[17]]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],5],[[],5],[[],5],[[]],[[]],[[]],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],18],[[],18],[[],18],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[],[[11,[4,15]]]],[[],[[11,[4]]]],[[],[[11,[4,1]]]],[[],[[11,[4,15]]]],[[],[[11,[4,1]]]],[[],[[11,[4,[3,[1,2]]]]]],[[],[[11,[4,1]]]],[[],[[11,[4,14]]]],0,0,0,0,0,0,[3],[3],[3],[3],[3],0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,0,[[],19],[[],20],[[],[[8,[21]]]],[[],[[8,[22]]]],[[],[[8,[19]]]],[[],[[8,[20]]]],[[],[[8,[23]]]],0,[[21,16],[[8,[17]]]],[[22,16],[[8,[17]]]],[[19,16],[[8,[17]]]],[[20,16],[[8,[17]]]],[[23,16],[[8,[17]]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,[[],5],[[],5],[[],5],[[],5],[[],5],0,0,0,0,0,[24,23],[[],23],0,0,0,0,[21,8],[22,8],[19,8],[20,8],[23,8],0,0,0,[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],8],[[],18],[[],18],[[],18],[[],18],[[],18],0,0,0,0,[25,[[8,[[3,[2]]]]]],[13,[[8,[[3,[2]]]]]]],"p":[[4,"Declaration"],[3,"Global"],[3,"Vec"],[15,"char"],[15,"usize"],[3,"Range"],[3,"Simple"],[4,"Result"],[3,"Report"],[3,"Box"],[8,"Parser"],[3,"Source"],[15,"str"],[4,"Statement"],[4,"Expression"],[3,"Formatter"],[3,"Error"],[3,"TypeId"],[3,"Project"],[3,"Meta"],[3,"Block"],[3,"Costume"],[3,"Target"],[3,"String"],[4,"Value"],[13,"Func"],[13,"Sprite"],[13,"Declare"],[13,"Number"],[13,"Text"],[13,"Boolean"],[13,"Call"],[13,"Call"]]},\
"claw_driver":{"doc":"<code>claw_driver</code>","t":[5,5],"n":["load_file","main"],"q":["claw_driver",""],"d":["Reads the contents of a <code>Path</code> to a <code>String</code>.",""],"i":[0,0],"f":[[1,[[3,[2]]]],[[]]],"p":[[3,"Path"],[3,"String"],[6,"Result"]]},\
"claw_middle":{"doc":"<code>claw_middle</code>","t":[0,0],"n":["stdlib","ty"],"q":["claw_middle",""],"d":["",""],"i":[0,0],"f":[0,0],"p":[]},\
"claw_parse":{"doc":"<code>claw_parse</code>","t":[2,6,0,5,2,5,5,0,6,13,13,13,4,13,4,13,13,13,4,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,12,5,5,5,5,5,5,5,5],"n":["AST","ParseError","ast","build_report","create_parser","get_source","parse","parser","AST","Boolean","Call","Call","Declaration","Declare","Expression","Func","Number","Sprite","Statement","Text","append_to","append_to","append_to","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone_into","clone_into","clone_into","fmt","fmt","fmt","from","from","from","get_iter","get_iter","get_iter","into","into","into","len","len","len","to_owned","to_owned","to_owned","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","body","declarations","events","items","kind","name","name","0","0","0","args","module","name","args","module","name","create_call","create_comment","create_declare","create_expression","create_func","create_parser","create_sprite","create_statement"],"q":["claw_parse","","","","","","","","claw_parse::ast","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","claw_parse::ast::Declaration","","","","","","","claw_parse::ast::Expression","","","","","","claw_parse::ast::Statement","","","claw_parse::parser","","","","","","",""],"d":["","Error type returned by the parsers.","Contains the abstract syntax tree definitions.","Transforms a result to use <code>ariadne</code>’s <code>Report</code>.","","Returns the <code>Source</code> of a string reference.","Parses a <code>str</code> into <code>AST</code>.","Contains the logic of the actual parser, written using …","Public type used to represent a complete program in AST …","A boolean.","A function call that does not return anything.","A function that returns a value.","Declares constants.","An enum declaration.","An expression to be used from within a <code>Statement</code>.","A function declaration.","A number, whether decimal or not.","A sprite declaration.","A statement that does not return a value.","A string of text.","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","The code that is run when the function is called.","The declarations within the sprite.","The events that trigger the function.","The items declared within the enum.","The kind of enum being declared.","The name of the function.","The name of the sprite.","","","","The arguments passed to the function.","The module the function is from.","The name of the function.","The arguments passed to the function.","The module the function is from.","The name of the function.","Creates a call parser.","Creates a comment parser.","Creates an enum declaration parser.","Creates an expression parser.","Creates a function declaration parser.","Creates the main <code>Parser</code> to be used when parsing a Claw …","Creates a sprite declaration parser.","Creates a statement parser."],"i":[0,0,0,0,0,0,0,0,0,12,11,12,0,10,0,10,12,10,0,12,10,11,12,10,11,12,10,11,12,10,11,12,10,11,12,10,11,12,10,11,12,10,11,12,10,11,12,10,11,12,10,11,12,10,11,12,10,11,12,10,11,12,19,20,19,21,21,19,20,22,23,24,25,25,25,26,26,26,0,0,0,0,0,0,0,0],"f":[0,0,0,[[[4,[1,[3,[2]]]]],[[4,[1,[6,[5]]]]]],0,[[[8,[7]]],9],[7,[[4,[1,[6,[5]]]]]],0,0,0,0,0,0,0,0,0,0,0,0,0,[3],[3],[3],[[]],[[]],[[]],[[]],[[]],[[]],[10,10],[11,11],[12,12],[[]],[[]],[[]],[[10,13],14],[[11,13],14],[[12,13],14],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],15],[[],15],[[],15],[[]],[[]],[[]],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],[[],16],[[],16],[[],16],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[],[[18,[17,12]]]],[[],[[18,[17]]]],[[],[[18,[17,10]]]],[[],[[18,[17,12]]]],[[],[[18,[17,10]]]],[[],[[18,[17,1]]]],[[],[[18,[17,10]]]],[[],[[18,[17,11]]]]],"p":[[6,"AST"],[6,"ParseError"],[3,"Vec"],[4,"Result"],[3,"Report"],[3,"Box"],[15,"str"],[8,"AsRef"],[3,"Source"],[4,"Declaration"],[4,"Statement"],[4,"Expression"],[3,"Formatter"],[6,"Result"],[15,"usize"],[3,"TypeId"],[15,"char"],[8,"Parser"],[13,"Func"],[13,"Sprite"],[13,"Declare"],[13,"Number"],[13,"Text"],[13,"Boolean"],[13,"Call"],[13,"Call"]]},\
"claw_sb3":{"doc":"<code>claw_sb3</code>","t":[3,3,3,3,3,12,12,12,11,11,11,11,11,11,11,11,11,11,12,12,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,11,11,12,12,12,12,11,11,11,11,11,12,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12],"n":["Block","Costume","Meta","Project","Target","agent","asset_id","blocks","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","costumes","data_format","default","default","deserialize","deserialize","deserialize","deserialize","deserialize","extensions","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","into","into","into","into","into","is_stage","md5ext","meta","monitors","name","name","new_sprite","new_stage","next","opcode","parent","semver","serialize","serialize","serialize","serialize","serialize","sounds","targets","top_level","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","variables","vm"],"q":["claw_sb3","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,0,0,0,2,5,6,3,5,1,2,6,3,5,1,2,6,6,5,1,2,3,5,1,2,6,1,3,5,1,2,6,3,5,1,2,6,3,5,1,2,6,6,5,1,1,5,6,6,6,3,3,3,2,3,5,1,2,6,6,1,3,3,5,1,2,6,3,5,1,2,6,3,5,1,2,6,6,2],"f":[0,0,0,0,0,0,0,0,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,0,[[],1],[[],2],[[],[[4,[3]]]],[[],[[4,[5]]]],[[],[[4,[1]]]],[[],[[4,[2]]]],[[],[[4,[6]]]],0,[[3,7],8],[[5,7],8],[[1,7],8],[[2,7],8],[[6,7],8],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],0,0,0,0,0,0,[9,6],[[],6],0,0,0,0,[3,4],[5,4],[1,4],[2,4],[6,4],0,0,0,[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],[[],4],[[],10],[[],10],[[],10],[[],10],[[],10],0,0],"p":[[3,"Project"],[3,"Meta"],[3,"Block"],[4,"Result"],[3,"Costume"],[3,"Target"],[3,"Formatter"],[6,"Result"],[3,"String"],[3,"TypeId"]]},\
"claw_verify":{"doc":"<code>claw_verify</code>","t":[17,17,5,5],"n":["SB3_DEFINITIONS","SB3_SCHEMA","verify","verify_string"],"q":["claw_verify","","",""],"d":["Shared definitions between multiple schemas, though right …","The schema that a <code>project.json</code> file is checked against.","The main function that verifies a <code>serde_json::Value</code> is a …","Conveniance function that verifies the string form of a …"],"i":[0,0,0,0],"f":[0,0,[1,[[3,[2]]]],[4,[[3,[2]]]]],"p":[[4,"Value"],[3,"Vec"],[4,"Result"],[15,"str"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
