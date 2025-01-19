
In the 2024-25 season of the UEFA champions league, a new league format was introduced to accomodate for the increase from 32 to 36 teams participating in the competition.

Details of the draw are as follows:

> All 36 teams will be manually drawn using physical balls. For every team manually drawn, a designated automated software will randomly draw eight opponents across the four pots, who will be revealed on screen in the draw hall and on television. The software will also decide which matches will be at home and which ones away.<sup>[[1]](https://www.uefa.com/news-media/news/028f-1b7f80216368-4c26e95cc15b-1000/)</sup>

This repo implements the 'designated automated software' mentioned above. 

# Rules
The following rules determine which opponents are compatible:

>To determine the eight different opponents, teams will be seeded in four pots ... Each team will then be drawn against two opponents from each pot, one of which will be at home and one away.
>In the league phase teams cannot face opponents from their country and can be drawn against a maximum of two sides from the same country.<sup>[[1]](https://www.uefa.com/news-media/news/028f-1b7f80216368-4c26e95cc15b-1000/)</sup>

pots taken from: https://www.uefa.com/uefachampionsleague/news/0290-1bb8f464468e-3d7a47f2094d-1000--champions-league-league-phase-draw-pots-confirmed/

# Program flow
1. separate teams into their 4 seeded pots
2. pick any random team (and remove from teams pool)
3. go through each pot and pick a random home and away fixture
4. add to fixture list
5. repeat from step 2 until teams pool is empty

## picking a random home and away fixture
we need to ensure that the fixtures is valid when a random team is drawn:
1. teams cannot play themselves 
2. the opoonent team must not already have the equivalent fixture with another team
   e.g. if we are drawing a home fixture for a pot 3 team, the opponent shouldnt already have an away fixture against another pot3 team
3. the opponent must not be from the same country
   e.g. a team from the premier league (england) cannot be drawn against another team from the premier league
4. teams cannot play more than two sides from the same country

to ensure validity when picking a random draw in [step 3 above](#program-flow), we need to have access to the following information;
- list of fixtures for the team we are drawing for
  - check we're not drawing the same team twice
  - check rule 4 for current team
- list of teams in the pot we are drawing for and their fixtures so far
  - check rule 4 for other teams


