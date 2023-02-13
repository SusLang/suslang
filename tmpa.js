// suslang automagically generated code
function report(s, ...a) {
	console.log(s.trimEnd(), a);
}

function ඬ(){
report("%d\n", lib_fibo_1(30));
return 0;
}
function lib_fibo_1(yellow){
if (yellow < 2) {
return yellow;
} else {
return lib_fibo_1(yellow - 1) + lib_fibo_1(yellow - 2);
}
}
ඬ()
