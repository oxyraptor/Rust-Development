vector and tuple 
a standard array have fixed size and can never grow or shrink
vector can change size dynamically usinf .insert() and .pop().
tuple group different data together

enum or enumeration 
a custom data type that allows to defing a value by listing its variants like in out case or snake game we have direction and its variants are up down right and left

match : match is a cotrol flow opetrator 
its exhastive meaning it matches or handel every single possible varians of an enum 

borrowing &
ownership is the most imp concept of rust 
it borrows the values 

grid system or adding grid to our window 
ive impliment or divide the window into grids though it drawn like they are just lines 
but they are important for future use
as we have to move the snake and randomly spawn the apple the grid is use full

to move the snake i figureout that i have to trace the path of the snake in a while loop and draw the sqaures from precious cordinateds to current cordinates after covering each bolck or cell of the grid 

rust is hard mf!!

connected data to the grid 
data like snake apple and score 
added the draw_rectangle in a loop and replaced the hardcoded numbers

turned snake into a list of tuple in a vector

snaped apple and snake to grid using offset x and y

got it made the snake move 
lesss gooo bc

thank god to all that DSA problems ive solved 
i can finally use the push and pop somewhere

now i just have to figure out how to steer the snake and make the apple swapn randomly 

if possible i might add wasd to steer snake 
ig that's it for today
man im so bad it took me 5 days to move the sanke which i cant even controll for now 

finally the game is completed 
today i added game state like game over and collision
and when the snake hit the other wall it will reappear from the opposite wall
and when the snake collide with itself the game will be over
the apple spawn randomly 
it uses queue for inputs to avoid reverse bug(if i press up and down before the clock ticks the snake will go reverse )

what i learned??
-> i learned the basics of rust like variable decleartion, loops, for loop, if else, integer, bool, float, vec, enum, etc

the most imp thing i learned is that how important the docs are like the docs and opensource projects helped me undertand rust 

well this journey was truly worth it, it helped me build confidance and now i think i can do better in rust or coding overall

ik no one is going to read this 

anyway more projects are on the way

i made apple green and snake read damn?!

