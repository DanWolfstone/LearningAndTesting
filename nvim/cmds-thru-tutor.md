# Learning Vim Commands and stuff

This is the type of place that I kinda need to have my hands on the home keys rather than how I normally have them I think? 
# MODES
esc	= Return to normal mode to enter cmds
a	= Append text after the line
i	= insert text before the cursor
v	= Highlighting
	y	= copy
	p	= paste
	d	= delete
	c	= cut

o	= open line below cursor -> into insert mode
	O	= opens line ABOVE cursor 
// A = I?? seems to do the same thing
d	= Delete motion | d Modifiers:
	w	= Deletes a word infront of your cursor
	e	= Deletes to the end of the word (Less useful)
	$	= deletes the end of a line
	<#>w	= Deletes # UPPERCASE words
	d | #dd	= Deletes a whole line (or x amt of lines)

<#>w	= move the cursor # words forward
0	= start of the line
$	= end of the line
e	= end of the word
u	= undo
U	= undo whole line 
ctrl+r	= Redo 
p	= paste line under cursor
R	=  enter replace mode
	r<>	= replace what's under cursor with whatever you type (good for single char)
c	= change motion | c modifiers:
	e	= until end of word
	$	= until end of line
	w	= entire word

## NORMAL MODE CMDS
!	= runs any external command
:q	= quit
:q!	= force quit
x/del	= Deletes text (under cursor(?))
:wq	= Save And Quit
:w	= write/save
	* can write highlighted portion to file
	* can write from lines x to y to file
:r 	= retrieve contents of thing and place below cursor
	<file>	= retrieve contents of file and insert
	!<cmd>	= retrieve output of cmd and insert below cursor

:set	= sets option from the config

G	= move to bottom of file
	#G	= move to that line in file
[[	= start of file
]]	= end of file

gg	= move to the start of file
/	= search
	n	= search last phrase 
	N	= search last phrase going up
	// ctrl+i/o works the same
	%	= Hover over ()|[]|{} and press % to find the closing bracket
	:set ic ignores case (noic resets that)

:s	= substitute in file
	Syntax: :%s/old/new/gc || :x,ys/old/new/gc
		% = changing every occurance in the file (without = acts on that line)
		g = globally (everything on that line/file)
		c = confirmation required
		xy= between lines x and y


