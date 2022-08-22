grammar all_test;

list: GET VAR_ ( ',' VAR_ )* ;
GET: 'get';
VAR_: '$' [a-zA-Z0-9][a-zA-Z0-9_-]* ;

WS : [ \t\r\n]+ -> channel(HIDDEN) ;
