Basic Queries:

	1. Write a SQL query to find the name and year of the movies. Return movie title, movie release year.
    
        select mov_title, mov_dt_rel from movie;

    2. Write a SQL query to find when the movie 'American Beauty' released. Return movie release year.
 
        select mov_dt_rel from movie where mov_title="American Beauty";

    3. Write a SQL query to find the movie that was released in 1999. Return movie title.

        select mov_title from movie where mov_year=1999;

    4. Write a SQL query to find those movies, which were released before 1998. Return movie title.

        select mov_title from movie where mov_year<1998;

    5. Write a SQL query to find the name of all reviewers and movies together in a single list.

        