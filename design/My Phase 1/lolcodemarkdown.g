grammar lolcodemarkdown;

//lexical definitions
HAI	:	'#' H A I;
KBYE	:	'#' K B Y E;
OBTW	:	'#' O B T W;
TLDR	:	'#' T L D R;
MAEK_HEAD	:	'#' M A E K ' ' H E A D; 
MAEK_PARAGRAF	:	'#' M A E K ' ' P A R A G R A F;
MAEK_LIST	:	'#' M A E K ' ' L I S T; 
MKAY	:	'#'M K A Y;
GIMMEH_TITLE	:	'#' G I M  M E H ' ' T I T L E;
GIMMEH_BOLD	:	'#' G I M M E H ' ' B O L D;
GIMMEH_ITEM	:	'#' G I M M E H ' ' I T E M;
GIMMEH_LINX	:	'#' G I M M E H ' ' L I N X;
GIMMEH_VAR	:	'#' G I M M E H ' ' V A R;
OIC	:	'#' O I C;
NEWLINE	:	'#' N E W L I N E; 
IHAZ	:	'#' I H A Z;
ITIZ	:	'#' I T I Z;

TEXT	:	('A'..'Z' | 'a'..'z' | '0'..'9' | ',' | '.' | '"' | ':' | '?' | '!' | '%' | '=' | '/')+;
WS	:	(' ' | '\t' | '\r' | '\n')+ { $channel = HIDDEN; };

fragment A	:	'a' | 'A';
fragment B	:	'b' | 'B';
fragment C	:	'c' | 'C';
fragment D	:	'd' | 'D';
fragment E	:	'e' | 'E';
fragment F	:	'f' | 'F';
fragment G	:	'g' | 'G';
fragment H	:	'h' | 'H';
fragment I	:	'i' | 'I';
fragment J	:	'j' | 'J';
fragment K	:	'k' | 'K';
fragment L	:	'l' | 'L';
fragment M	:	'm' | 'M';
fragment N	:	'n' | 'N';
fragment O	:	'o' | 'O';
fragment P	:	'p' | 'P';
fragment Q	:	'q' | 'Q';
fragment R	:	'r' | 'R';
fragment S	:	's' | 'S';
fragment T	:	't' | 'T';
fragment U	:	'u' | 'U';
fragment V	:	'v' | 'V';
fragment W	:	'w' | 'W';
fragment X	:	'x' | 'X';
fragment Y	:	'y' | 'Y';
fragment Z	:	'z' | 'Z';

//syntax definitions
lolcode	:	 HAI lolcode_start KBYE EOF;
lolcode_start	:	var_def lolcode_content | lolcode_content;
lolcode_content	:	comment lolcode_content | head body | start_content body | ;
start_content	:	paragraph | list | bold | link | newline | var_use | TEXT;

body	:	body_content body | ;
body_content	:	paragraph | inner_content;
inner_content	:	comment | list | bold | link | newline | var_use | TEXT;

head	:	MAEK_HEAD head_start MKAY;
head_start	:	var_def head_title | head_title;
head_title	:	comment head_title | title head_end;
head_end	:	comment head_end | ;
title	:	GIMMEH_TITLE text OIC;

paragraph	:	MAEK_PARAGRAF para_start MKAY;
para_start	:	var_def para_content | para_content;
para_content	:	inner_content para_content | ;

bold	:	GIMMEH_BOLD bold_start OIC;
bold_start	:	var_def inner_text | inner_text;
inner_text	:	text_item inner_text | ;
text_item	:	var_use | TEXT;

list	:	MAEK_LIST list_start MKAY;
list_start	:	var_def list_item list_more | list_item list_more;
list_more	:	list_item list_more | ;
list_item	:	GIMMEH_ITEM item_start OIC;
item_start	:	var_def inner_list | inner_list;
inner_list	:	list_part inner_list | ;
list_part	:	bold | var_use | TEXT;
	
comment	:	OBTW text TLDR;
link	:	GIMMEH_LINX TEXT OIC;
newline	:	NEWLINE;
var_def	:	IHAZ TEXT ITIZ text MKAY;
var_use	:	GIMMEH_VAR TEXT OIC;
text	:	TEXT more_text;
more_text	:	TEXT more_text | ;