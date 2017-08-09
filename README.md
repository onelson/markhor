# Markhor Checklist

So, this all grew out of 
[this thread on twitter](https://twitter.com/theomn/status/894434075854872578)
where I cited some concerns about tracking progress towards some trophies in 
Metal Gear Solid 3: Snake Eater (for the PS3 HD release).

The "King of the Jungle" trophy is a superset of several other trophies in the
game where the player must acquire one of each type of _thing_ in a category.

Collecting all of each thing in each category will qualify the player for the 
rank of Markhor, hence the name of this app.

I am yet to find a nice (single) list of all the stuff to be tracked down, but 
one article mentions the number of distinct items is 48 (but doesn't list what
they all are).

I plan to go through 
[these sub-lists](http://metalgear.wikia.com/wiki/Food_(Metal_Gear_Solid_3)) 
and check the count after tallying them all up. Still, this is orthogonal to 
the problem the app aims to solve which is tracking progress.

## Design Concerns

For the purpose of this discussion, "item" refers to a distinct class of thing 
that can be collected in the game world which will mark progress towards the 
trophy goal.

The current assumption is that all items are food items (contribution towards 
Markhor) but this design should look towards other collection-style trophies
such as uniforms, or face camo which are similar to the Mankhor challenge, but
they just number far fewer in the game.

Additionally, food items should: 

* have a group (eg Snake, Bird, Fruit, etc).
* have a short and long name (the wiki refers to the short name as the ration 
  name, ex Fish A, Snake K).
* be known to be found in a list of game "zones".

Zones are a bit of a hairy subject since they represent sections of the game 
world, but may also have a time or event trigger associated with them. For 
example there are 2 separate phases in the game where you visit the 
Groznyj Grad fortress, and item availability may differ in each phase. As such,
This zone may need 2 distinct entries in the data store to differentiate them.

The interface for the app should allow the player to filter by zone
specifically, and allow a short list of at a glance counts for the item groups
for items available in that given zone. 

It should also allow for comprehensive tracking (all groups, all zones), as 
well as drill down by group (potentially showing zone info with each item 
listed, though this might be tricky in terms of layout).
