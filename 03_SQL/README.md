Basic Queries:

	1. Write a SQL query to find the name and year of the movies. Return movie title, movie release year.
    
        select mov_title, mov_dt_rel from movie;

        +--------------------------+------------+
        | mov_title                | mov_dt_rel |
        +--------------------------+------------+
        | Vertigo                  | 1958-08-24 |
        | The Innocents            | 1962-02-19 |
        | Lawrence of Arabia       | 1962-12-11 |
        | The Deer Hunter          | 1979-03-08 |
        | Amadeus                  | 1985-01-07 |
        | Blade Runner             | 1982-09-09 |
        | Eyes Wide Shut           | NULL       |
        | The Usual Suspects       | 1995-08-25 |
        | Chinatown                | 1974-08-09 |
        | Boogie Nights            | 1998-02-16 |
        | Annie Hall               | 1977-04-20 |
        | Princess Mononoke        | 2001-10-19 |
        | The Shawshank Redemption | 1995-02-17 |
        | American Beauty          | NULL       |
        | Titanic                  | 1998-01-23 |
        | Good Will Hunting        | 1998-06-03 |
        | Deliverance              | 1982-10-05 |
        | Trainspotting            | 1996-02-23 |
        | The Prestige             | 2006-11-10 |
        | Donnie Darko             | NULL       |
        | Slumdog Millionaire      | 2009-01-09 |
        | Aliens                   | 1986-08-29 |
        | Beyond the Sea           | 2004-11-26 |
        | Avatar                   | 2009-12-17 |
        | Seven Samurai            | 1954-04-26 |
        | Spirited Away            | 2003-09-12 |
        | Back to the Future       | 1985-12-04 |
        | Braveheart               | 1995-09-08 |
        +--------------------------+------------+

    2. Write a SQL query to find when the movie 'American Beauty' released. Return movie release year.
 
        select mov_dt_rel from movie where mov_title="American Beauty";

        +------------+
        | mov_dt_rel |
        +------------+
        | NULL       |
        +------------+

    3. Write a SQL query to find the movie that was released in 1999. Return movie title.

        select mov_title from movie where mov_year=1999;

        +-----------------+
        | mov_title       |
        +-----------------+
        | Eyes Wide Shut  |
        | American Beauty |
        +-----------------+

    4. Write a SQL query to find those movies, which were released before 1998. Return movie title.

        select mov_title from movie where mov_year<1998;

        +--------------------------+
        | mov_title                |
        +--------------------------+
        | Vertigo                  |
        | The Innocents            |
        | Lawrence of Arabia       |
        | The Deer Hunter          |
        | Amadeus                  |
        | Blade Runner             |
        | The Usual Suspects       |
        | Chinatown                |
        | Boogie Nights            |
        | Annie Hall               |
        | Princess Mononoke        |
        | The Shawshank Redemption |
        | Titanic                  |
        | Good Will Hunting        |
        | Deliverance              |
        | Trainspotting            |
        | Aliens                   |
        | Seven Samurai            |
        | Back to the Future       |
        | Braveheart               |
        +--------------------------+

    5. Write a SQL query to find the name of all reviewers and movies together in a single list.

        select mov_title from movie union select rev_name from movie_reviewer;

        +--------------------------+
        | mov_title                |
        +--------------------------+
        | Vertigo                  |
        | The Innocents            |
        | Lawrence of Arabia       |
        | The Deer Hunter          |
        | Amadeus                  |
        | Blade Runner             |
        | Eyes Wide Shut           |
        | The Usual Suspects       |
        | Chinatown                |
        | Boogie Nights            |
        | Annie Hall               |
        | Princess Mononoke        |
        | The Shawshank Redemption |
        | American Beauty          |
        | Titanic                  |
        | Good Will Hunting        |
        | Deliverance              |
        | Trainspotting            |
        | The Prestige             |
        | Donnie Darko             |
        | Slumdog Millionaire      |
        | Aliens                   |
        | Beyond the Sea           |
        | Avatar                   |
        | Seven Samurai            |
        | Spirited Away            |
        | Back to the Future       |
        | Braveheart               |
        | Righty Sock              |
        | Jack Malvern             |
        | Flagrant Baronessa       |
        | Alec Shaw                |
        | NULL                     |
        | Victor Woeltjen          |
        | Simon Wright             |
        | Neal Wruck               |
        | Paul Monks               |
        | Mike Salvati             |
        | Wesley S. Walker         |
        | Sasha Goldshtein         |
        | Josh Cates               |
        | Krug Stillo              |
        | Scott LeBrun             |
        | Hannah Steele            |
        | Vincent Cadena           |
        | Brandt Sponseller        |
        | Richard Adams            |
        +--------------------------+

    6. Write a SQL query to find all reviewers who have rated seven or more stars to their rating. Return reviewer name.

        select rv.rev_name 
        from movie_reviewer as rv
        left join movie_rating as r on r.rev_id = rv.rev_id;

        +--------------------+
        | rev_name           |
        +--------------------+
        | Righty Sock        |
        | Righty Sock        |
        | Jack Malvern       |
        | Flagrant Baronessa |
        | Alec Shaw          |
        | NULL               |
        | Victor Woeltjen    |
        | Simon Wright       |
        | Neal Wruck         |
        | Paul Monks         |
        | Mike Salvati       |
        | NULL               |
        | Wesley S. Walker   |
        | Sasha Goldshtein   |
        | Josh Cates         |
        | Krug Stillo        |
        | Scott LeBrun       |
        | Hannah Steele      |
        | Vincent Cadena     |
        | Brandt Sponseller  |
        | Richard Adams      |
        +--------------------+

    7. Write a SQL query to find the movies without any rating. Return movie title.

        select m.mov_title 
        from movie as m
        left join movie_rating as r on r.mov_id = m.mov_id
        where r.rev_stars IS NULL;

        +--------------------------+
        | mov_title                |
        +--------------------------+
        | The Deer Hunter          |
        | Amadeus                  |
        | Eyes Wide Shut           |
        | Chinatown                |
        | The Shawshank Redemption |
        | Deliverance              |
        | Trainspotting            |
        | The Prestige             |
        | Spirited Away            |
        | Back to the Future       |
        | Braveheart               |
        +--------------------------+

    8. Write a SQL query to find the movies with ID 905 or 907 or 917. Return movie title.

        select mov_title from movie where mov_id IN (905,907,917);

    9. Write a SQL query to find the movie titles that contain the word 'Boogie Nights'. Sort the result-set in ascending order by movie year. Return movie ID, movie title and movie release year.

        select mov_id, mov_title, mov_year from movie where mov_title like "Boogie Nights%" order by mov_year asc;

        +--------+---------------+----------+
        | mov_id | mov_title     | mov_year |
        +--------+---------------+----------+
        |     10 | Boogie Nights |     1997 |
        +--------+---------------+----------+

    10. Write a SQL query to find those actors with the first name 'Woody' and the last name 'Allen'. Return actor ID.

        select act_id from actor where act_fname="Woody" and acrt_lname="Allen";

        +--------+
        | act_id |
        +--------+
        |     11 |
        +--------+


Sub-Queries :


    11. Write a SQL query to find the actors who played a role in the movie 'Annie Hall'. Return all the fields of actor table.

        select * from actor where act_id in (select act_id from movie_cast where mov_id in (select mov_id from movie where mov_title = "Annie Hall"));

        +--------+-----------+------------+------------+
        | act_id | act_fname | acrt_lname | act_gender |
        +--------+-----------+------------+------------+
        |     11 | Woody     | Allen      | M          |
        +--------+-----------+------------+------------+

    12. Write a SQL query to find the director of a film that cast a role in 'Eyes Wide Shut'. Return director first name, last name.

        select dir_fname, dir_lname from director where dir_id in (select dir_id from movie_direction where mov_id in (select mov_id from movie where mov_title = "Eyes Wide Shut"));

        +-----------+-----------+
        | dir_fname | dir_lname |
        +-----------+-----------+
        | Stanley   | Kubrick   |
        +-----------+-----------+

    13. Write a SQL query to find those movies that have been released in countries other than the United Kingdom. Return movie title, movie year, movie time, and date of release, releasing country.

        select mov_title, mov_year, mov_time, mov_dt_rel, mov_rel_country from movie where mov_rel_country not in ("UK");

        +---------------+----------+----------+------------+-----------------+
        | mov_title     | mov_year | mov_time | mov_dt_rel | mov_rel_country |
        +---------------+----------+----------+------------+-----------------+
        | The Innocents |     1961 |      100 | 1962-02-19 | SW              |
        | Annie Hall    |     1977 |       93 | 1977-04-20 | USA             |
        | Seven Samurai |     1954 |      207 | 1954-04-26 | JP              |
        +---------------+----------+----------+------------+-----------------+

    14. Write a SQL query to find for movies whose reviewer is unknown. Return movie title, year, release date, director first name, last name, actor first name, last name.

        select mov_title, mov_year, mov_dt_rel, dir_fname, dir_lname, act_fname, acrt_lname
        from movie
        left join movie_direction on movie_direction.mov_id = movie.mov_id
        left join director on director.dir_id = movie_direction.dir_id
        left join movie_cast on movie_cast.mov_id = movie.mov_id
        left join actor on actor.act_id = movie_cast.act_id
        where movie.mov_id in (select mov_id from movie where mov_id in (select mov_id from movie_rating where rev_id in (select rev_id from movie_reviewer where rev_name is Null)));

        +-------------------+----------+------------+-----------+-----------+-----------+------------+
        | mov_title         | mov_year | mov_dt_rel | dir_fname | dir_lname | act_fname | acrt_lname |
        +-------------------+----------+------------+-----------+-----------+-----------+------------+
        | Blade Runner      |     1982 | 1982-09-09 | Ridley    | Scott     | Harrison  | Ford       |
        | Princess Mononoke |     1997 | 2001-10-19 | Hayao     | Miyazaki  | Claire    | Danes      |
        +-------------------+----------+------------+-----------+-----------+-----------+------------+

    15. Write a SQL query to find those movies directed by the director whose first name is Woddy and last name is Allen. Return movie title.

        select mov_title from movie where mov_id in (select mov_id from movie_direction where dir_id in (select dir_id from director where dir_fname = "Woddy" and dir_lname = "Allen"));

    16. Write a SQL query to determine those years in which there was at least one movie that received a rating of at least three stars. Sort the result-set in ascending order by movie year. Return movie year.

        select mov_year from movie where mov_id in (select mov_id from movie_rating where num_o_ratings>=1 and rev_stars>=3) order by mov_year asc;

        +----------+
        | mov_year |
        +----------+
        |     1954 |
        |     1958 |
        |     1961 |
        |     1962 |
        |     1977 |
        |     1982 |
        |     1986 |
        |     1995 |
        |     1997 |
        |     1997 |
        |     1997 |
        |     1999 |
        |     2001 |
        |     2004 |
        |     2008 |
        +----------+

    17. Write a SQL query to search for movies that do not have any ratings. Return movie title.

        select mov_title from movie where mov_id in (select mov_id from movie_rating where num_o_ratings is Null);

        +-------------------+
        | mov_title         |
        +-------------------+
        | Avatar            |
        | Princess Mononoke |
        +-------------------+

    18. Write a SQL query to find those reviewers who have not given a rating to certain films. Return reviewer name.

        select rev_name from movie_reviewer where rev_id in (select rev_id from movie_rating where rev_stars is Null);

        +--------------+
        | rev_name     |
        +--------------+
        | Neal Wruck   |
        | Scott LeBrun |
        +--------------+
    
    19. Write a SQL query to find movies that have been reviewed by a reviewer and received a rating. Sort the result-set in ascending order by reviewer name, movie title, review Stars. Return reviewer name, movie title, review Stars.

        select
        (select mov_title from movie m where m.mov_id = mr.mov_id) as movie_title,
        (select rev_name from movie_reviewer mer where mer.rev_id = mr.rev_id) as reviewer_name, 
        rev_stars
        from movie_rating mr
        where rev_stars IS NOT NULL
        ORDER BY movie_title ASC, reviewer_name ASC, rev_stars ASC;

        +---------------------+--------------------+-----------+
        | movie_title         | reviewer_name      | rev_stars |
        +---------------------+--------------------+-----------+
        | Aliens              | Brandt Sponseller  |       8.4 |
        | American Beauty     | Sasha Goldshtein   |       7.0 |
        | Annie Hall          | Mike Salvati       |       8.1 |
        | Avatar              | Victor Woeltjen    |       7.3 |
        | Beyond the Sea      | Richard Adams      |       6.7 |
        | Blade Runner        | NULL               |       8.2 |
        | Boogie Nights       | Paul Monks         |       3.0 |
        | Donnie Darko        | Hannah Steele      |       8.1 |
        | Good Will Hunting   | Josh Cates         |       4.0 |
        | Lawrence of Arabia  | Flagrant Baronessa |       8.3 |
        | Princess Mononoke   | NULL               |       8.4 |
        | Seven Samurai       | Krug Stillo        |       7.7 |
        | Slumdog Millionaire | Vincent Cadena     |       8.0 |
        | The Innocents       | Jack Malvern       |       7.9 |
        | The Usual Suspects  | Simon Wright       |       8.6 |
        | Titanic             | Righty Sock        |       7.7 |
        | Vertigo             | Righty Sock        |       8.4 |
        +---------------------+--------------------+-----------+

	20. Write a SQL query to find movies that have been reviewed by a reviewer and received a rating. Group the result set on reviewer’s name, movie title. Return reviewer’s name, movie title.

        select
        (select mov_title from movie m where m.mov_id = mr.mov_id) as movie_title,
        (select rev_name from movie_reviewer mer where mer.rev_id = mr.rev_id) as reviewer_name
        from movie_rating mr
        where mr.rev_stars IS NOT NULL
        GROUP BY movie_title, reviewer_name;

        +---------------------+--------------------+
        | movie_title         | reviewer_name      |
        +---------------------+--------------------+
        | Vertigo             | Righty Sock        |
        | The Innocents       | Jack Malvern       |
        | Lawrence of Arabia  | Flagrant Baronessa |
        | Blade Runner        | NULL               |
        | Avatar              | Victor Woeltjen    |
        | The Usual Suspects  | Simon Wright       |
        | Boogie Nights       | Paul Monks         |
        | Annie Hall          | Mike Salvati       |
        | Princess Mononoke   | NULL               |
        | American Beauty     | Sasha Goldshtein   |
        | Titanic             | Righty Sock        |
        | Good Will Hunting   | Josh Cates         |
        | Seven Samurai       | Krug Stillo        |
        | Donnie Darko        | Hannah Steele      |
        | Slumdog Millionaire | Vincent Cadena     |
        | Aliens              | Brandt Sponseller  |
        | Beyond the Sea      | Richard Adams      |
        +---------------------+--------------------+

	21. Write a SQL query to find those movies, which have received highest number of stars. Group the result set on movie title and sorts the result-set in ascending order by movie title. Return movie title and maximum number of review stars.

        select m.mov_title, r.rev_stars AS max_stars
        from movie_rating r
        join movie m on r.mov_id = m.mov_id
        where r.rev_stars = ( select MAX(rev_stars) from movie_rating )
        order by m.mov_title ASC;

	22. Write a SQL query to find all reviewers who rated the movie 'American Beauty'. Return reviewer name.

        select rev_name from movie_reviewer 
        where rev_id = (select rev_id from movie_rating where mov_id = (select mov_id from movie where mov_title = 'American Beauty'));

        +------------------+
        | rev_name         |
        +------------------+
        | Sasha Goldshtein |
        +------------------+

	23. Write a SQL query to find the movies that have not been reviewed by any reviewer body other than 'Paul Monks'. Return movie title.

        select * from movie 
        where mov_id = (select mov_id from movie_rating where rev_id = (select rev_id from movie_reviewer where rev_name = 'Paul Monks'));

        +--------+---------------+----------+----------+----------+------------+-----------------+
        | mov_id | mov_title     | mov_year | mov_time | mov_lang | mov_dt_rel | mov_rel_country |
        +--------+---------------+----------+----------+----------+------------+-----------------+
        |     10 | Boogie Nights |     1997 |      155 | English  | 1998-02-16 | UK              |
        +--------+---------------+----------+----------+----------+------------+-----------------+

	24. Write a SQL query to find the movies with the lowest ratings. Return reviewer name, movie title, and number of stars for those movies.

        select m.mov_title, mr.rev_name from movie m
        join( 
                select * from movie_rating
                where rev_stars = ( select MIN(rev_stars) from movie_rating )
            ) AS min_rating
        on m.mov_id = min_rating.mov_id
        join movie_reviewer mr on mr.rev_id = min_rating.rev_id;

	25. Write a SQL query to find the movies directed by 'James Cameron'. Return movie title.

        select mov_title from movie 
        where mov_id = 
            (select mov_id from movie_direction where mov_id = 
                (select dir_id from movie_direction where dir_id = "James Cameron")
            );


	26. Write a query in SQL to find the movies in which one or more actors appeared in more than one film.

        select mov_title from movie 
        where mov_id in
            (SELECT mov_id FROM movie_cast
            WHERE mov_id IN (
                SELECT mov_id
                FROM movie_cast
                GROUP BY mov_id
                HAVING COUNT(*) > 1
            )
        );


Joins :

	27. Write a SQL query to find all reviewers whose ratings contain a NULL value. Return reviewer name.

        select * from movie_reviewer as rv 
        right join movie_rating as r on r.rev_id = rv.rev_id 
        where r.rev_stars is Null;

        +--------+--------------+--------+--------+-----------+---------------+
        | rev_id | rev_name     | mov_id | rev_id | rev_stars | num_o_ratings |
        +--------+--------------+--------+--------+-----------+---------------+
        |      8 | Neal Wruck   |      9 |      8 |      NULL |        227235 |
        |     16 | Scott LeBrun |     18 |     16 |      NULL |        580301 |
        +--------+--------------+--------+--------+-----------+---------------+

	28. Write a SQL query to find out who was cast in the movie 'Annie Hall'. Return actor first name, last name and role.

        select actor.act_fname, actor.acrt_lname, movie_cast.role from movie_cast 
        left join actor on actor.act_id = movie_cast.act_id
        left join movie on movie.mov_id = movie_cast.mov_id
        where mov_title = "Annie Hall";

        +-----------+------------+-------------+
        | act_fname | acrt_lname | role        |
        +-----------+------------+-------------+
        | Woody     | Allen      | Alvy Singer |
        +-----------+------------+-------------+

	29. Write a SQL query to find the director who directed a movie that featured a role in 'Eyes Wide Shut'. Return director first name, last name and movie title.

        select director.dir_fname, director.dir_lname, movie.mov_title from movie_direction 
        left join director on director.dir_id = movie_direction.dir_id
        left join movie on movie.mov_id = movie_direction.mov_id
        where mov_title = "Eyes Wide Shut";

        +-----------+-----------+----------------+
        | dir_fname | dir_lname | mov_title      |
        +-----------+-----------+----------------+
        | Stanley   | Kubrick   | Eyes Wide Shut |
        +-----------+-----------+----------------+

	30. Write a SQL query to find the director of a movie that cast a role as Sean Maguire. Return director first name, last name and movie title.

        select director.dir_fname, director.dir_lname, movie.mov_title from movie_direction 
        left join director on director.dir_id = movie_direction.dir_id
        left join movie on movie.mov_id = movie_direction.mov_id
        left join movie_cast on movie_cast.mov_id = movie.mov_id
        where movie_cast.role = "Sean Maguire";

        +-----------+-----------+-------------------+
        | dir_fname | dir_lname | mov_title         |
        +-----------+-----------+-------------------+
        | Gus       | Van Sant  | Good Will Hunting |
        +-----------+-----------+-------------------+

	31. Write a SQL query to find out which actors have not appeared in any movies between 1990 and 2000 (Begin and end values are included.). Return actor first name, last name, movie title and release year.

        SELECT a.act_fname, a.acrt_lname, m.mov_title, m.mov_year
        FROM actor a
        LEFT JOIN movie_cast mc ON a.act_id = mc.act_id
        LEFT JOIN movie m ON mc.mov_id = m.mov_id
        WHERE a.act_id NOT IN (
            SELECT mc2.act_id
            FROM movie_cast mc2
            JOIN movie m2 ON mc2.mov_id = m2.mov_id
            WHERE m2.mov_year BETWEEN 1990 AND 2000
        );

        +-----------+------------+---------------------+----------+
        | act_fname | acrt_lname | mov_title           | mov_year |
        +-----------+------------+---------------------+----------+
        | James     | Stewart    | Vertigo             |     1958 |
        | Deborah   | Kerr       | The Innocents       |     1961 |
        | Peter     | OToole     | Lawrence of Arabia  |     1962 |
        | Robert    | De Niro    | The Deer Hunter     |     1978 |
        | F. Murray | Abraham    | Amadeus             |     1984 |
        | Harrison  | Ford       | Blade Runner        |     1982 |
        | Jack      | Nicholson  | Chinatown           |     1974 |
        | Woody     | Allen      | Annie Hall          |     1977 |
        | Jon       | Voight     | Deliverance         |     1972 |
        | Christian | Bale       | Chinatown           |     1974 |
        | Maggie    | Gyllenhaal | Donnie Darko        |     2001 |
        | Dev       | Patel      | Slumdog Millionaire |     2008 |
        | Sigourney | Weaver     | Aliens              |     1986 |
        | David     | Aston      | NULL                |     NULL |
        | Ali       | Astin      | NULL                |     NULL |
        +-----------+------------+---------------------+----------+

	32. Write a SQL query to find the directors who have directed films in a variety of genres. Group the result set on director first name, last name and generic title. Sort the result-set in ascending order by director first name and last name. Return director first name, last name and number of genres movies.

        SELECT d.dir_fname, d.dir_lname, COUNT(DISTINCT g.gen_id) AS number_of_genres
        FROM director d
        JOIN movie_direction md ON d.dir_id = md.dir_id
        JOIN movie_genres mg ON md.mov_id = mg.mov_id
        JOIN genres g ON mg.gen_id = g.gen_id
        GROUP BY d.dir_fname, d.dir_lname
        ORDER BY d.dir_fname ASC, d.dir_lname ASC;

        +-----------+-----------+------------------+
        | dir_fname | dir_lname | number_of_genres |
        +-----------+-----------+------------------+
        | Alfred    | Hitchcock |                1 |
        | Bryan     | Singer    |                1 |
        | Danny     | Boyle     |                1 |
        | David     | Lean      |                1 |
        | Frank     | Darabont  |                1 |
        | Hayao     | Miyazaki  |                1 |
        | Jack      | Clayton   |                1 |
        | James     | Cameron   |                1 |
        | John      | Boorman   |                1 |
        | Kevin     | Spacey    |                1 |
        | Michael   | Cimino    |                1 |
        | Ridley    | Scott     |                1 |
        | Sam       | Mendes    |                1 |
        | Stanley   | Kubrick   |                1 |
        | Woody     | Allen     |                1 |
        +-----------+-----------+------------------+

	33. Write a SQL query to find the movies with year and genres. Return movie title, movie year and generic title.

        SELECT m.mov_title, m.mov_year, g.gen_title
        FROM movie m
        JOIN movie_genres mg ON m.mov_id = mg.mov_id
        JOIN genres g ON mg.gen_id = g.gen_id;

        +--------------------------+----------+-----------+
        | mov_title                | mov_year | gen_title |
        +--------------------------+----------+-----------+
        | Aliens                   |     1986 | Action    |
        | Deliverance              |     1972 | Adventure |
        | Lawrence of Arabia       |     1962 | Adventure |
        | Princess Mononoke        |     1997 | Animation |
        | Annie Hall               |     1977 | Comedy    |
        | The Usual Suspects       |     1995 | Crime     |
        | The Shawshank Redemption |     1994 | Crime     |
        | Spirited Away            |     2001 | Drama     |
        | Braveheart               |     1995 | Drama     |
        | Trainspotting            |     1996 | Drama     |
        | Slumdog Millionaire      |     2008 | Drama     |
        | The Innocents            |     1961 | Horror    |
        | Beyond the Sea           |     2004 | Music     |
        | Eyes Wide Shut           |     1999 | Mystery   |
        | Back to the Future       |     1985 | Mystery   |
        | Vertigo                  |     1958 | Mystery   |
        | American Beauty          |     1999 | Romance   |
        | Blade Runner             |     1982 | Thriller  |
        | The Deer Hunter          |     1978 | War       |
        +--------------------------+----------+-----------+

	34. Write a SQL query to find all the movies with year, genres, and name of the director.

        SELECT m.mov_title, m.mov_year, g.gen_title, d.dir_fname, d.dir_lname
        FROM movie m
        JOIN movie_genres mg ON m.mov_id = mg.mov_id
        JOIN genres g ON mg.gen_id = g.gen_id
        JOIN movie_direction md ON m.mov_id = md.mov_id
        JOIN director d ON md.dir_id = d.dir_id;

        +--------------------------+----------+-----------+-----------+-----------+
        | mov_title                | mov_year | gen_title | dir_fname | dir_lname |
        +--------------------------+----------+-----------+-----------+-----------+
        | Aliens                   |     1986 | Action    | James     | Cameron   |
        | Deliverance              |     1972 | Adventure | John      | Boorman   |
        | Lawrence of Arabia       |     1962 | Adventure | David     | Lean      |
        | Princess Mononoke        |     1997 | Animation | Hayao     | Miyazaki  |
        | Annie Hall               |     1977 | Comedy    | Woody     | Allen     |
        | The Usual Suspects       |     1995 | Crime     | Bryan     | Singer    |
        | The Shawshank Redemption |     1994 | Crime     | Frank     | Darabont  |
        | Trainspotting            |     1996 | Drama     | Danny     | Boyle     |
        | Slumdog Millionaire      |     2008 | Drama     | Danny     | Boyle     |
        | The Innocents            |     1961 | Horror    | Jack      | Clayton   |
        | Beyond the Sea           |     2004 | Music     | Kevin     | Spacey    |
        | Eyes Wide Shut           |     1999 | Mystery   | Stanley   | Kubrick   |
        | Vertigo                  |     1958 | Mystery   | Alfred    | Hitchcock |
        | American Beauty          |     1999 | Romance   | Sam       | Mendes    |
        | Blade Runner             |     1982 | Thriller  | Ridley    | Scott     |
        | The Deer Hunter          |     1978 | War       | Michael   | Cimino    |
        +--------------------------+----------+-----------+-----------+-----------+

	35. Write a SQL query to find the movies released before 1st January 1989. Sort the result-set in descending order by date of release. Return movie title, release year, date of release, duration, and first and last name of the director.

        SELECT m.mov_title, m.mov_year, m.mov_dt_rel, m.mov_time, d.dir_fname, d.dir_lname
        FROM movie m
        JOIN movie_direction md ON m.mov_id = md.mov_id
        JOIN director d ON md.dir_id = d.dir_id
        WHERE m.mov_dt_rel < '1989-01-01'
        ORDER BY m.mov_dt_rel DESC;

        +--------------------+----------+------------+----------+-----------+-----------+
        | mov_title          | mov_year | mov_dt_rel | mov_time | dir_fname | dir_lname |
        +--------------------+----------+------------+----------+-----------+-----------+
        | Aliens             |     1986 | 1986-08-29 |      137 | James     | Cameron   |
        | Amadeus            |     1984 | 1985-01-07 |      160 | Milos     | Forman    |
        | Deliverance        |     1972 | 1982-10-05 |      109 | John      | Boorman   |
        | Blade Runner       |     1982 | 1982-09-09 |      117 | Ridley    | Scott     |
        | The Deer Hunter    |     1978 | 1979-03-08 |      183 | Michael   | Cimino    |
        | Annie Hall         |     1977 | 1977-04-20 |       93 | Woody     | Allen     |
        | Chinatown          |     1974 | 1974-08-09 |      130 | Roman     | Polanski  |
        | Lawrence of Arabia |     1962 | 1962-12-11 |      216 | David     | Lean      |
        | The Innocents      |     1961 | 1962-02-19 |      100 | Jack      | Clayton   |
        | Vertigo            |     1958 | 1958-08-24 |      128 | Alfred    | Hitchcock |
        +--------------------+----------+------------+----------+-----------+-----------+

	36. Write a SQL query to calculate the average movie length and count the number of movies in each genre. Return genre title, average time and number of movies for each genre.

        SELECT g.gen_title, AVG(m.mov_time) AS avg_movie_length, COUNT(m.mov_id) AS number_of_movies
        FROM genres g
        JOIN movie_genres mg ON g.gen_id = mg.gen_id
        JOIN movie m ON mg.mov_id = m.mov_id
        GROUP BY g.gen_title;

        +-----------+------------------+------------------+
        | gen_title | avg_movie_length | number_of_movies |
        +-----------+------------------+------------------+
        | Action    |         137.0000 |                1 |
        | Adventure |         162.5000 |                2 |
        | Animation |         134.0000 |                1 |
        | Comedy    |          93.0000 |                1 |
        | Crime     |         124.0000 |                2 |
        | Drama     |         129.2500 |                4 |
        | Horror    |         100.0000 |                1 |
        | Music     |         118.0000 |                1 |
        | Mystery   |         134.3333 |                3 |
        | Romance   |         122.0000 |                1 |
        | Thriller  |         117.0000 |                1 |
        | War       |         183.0000 |                1 |
        +-----------+------------------+------------------+

	37. Write a SQL query to find movies with the shortest duration. Return movie title, movie year, director first name, last name, actor first name, last name and role.

        SELECT m.mov_title, m.mov_year, d.dir_fname, d.dir_lname, a.act_fname, a.acrt_lname, mc.role
        FROM movie m
        JOIN movie_direction md ON m.mov_id = md.mov_id
        JOIN director d ON md.dir_id = d.dir_id
        JOIN movie_cast mc ON m.mov_id = mc.mov_id
        JOIN actor a ON mc.act_id = a.act_id
        WHERE m.mov_time = (
            SELECT MIN(mov_time)
            FROM movie
        );

        +------------+----------+-----------+-----------+-----------+------------+-------------+
        | mov_title  | mov_year | dir_fname | dir_lname | act_fname | acrt_lname | role        |
        +------------+----------+-----------+-----------+-----------+------------+-------------+
        | Annie Hall |     1977 | Woody     | Allen     | Woody     | Allen      | Alvy Singer |
        +------------+----------+-----------+-----------+-----------+------------+-------------+

	38. Write a SQL query to find the years in which a movie received a rating of 3 or 4. Sort the result in increasing order on movie year.

        SELECT DISTINCT m.mov_year
        FROM movie m
        JOIN movie_rating mr ON m.mov_id = mr.mov_id
        WHERE mr.rev_stars IN (3, 4)
        ORDER BY m.mov_year ASC;

        +----------+
        | mov_year |
        +----------+
        |     1997 |
        +----------+

	39. Write a SQL query to get the reviewer name, movie title, and stars in an order that reviewer name will come first, then by movie title, and lastly by number of stars.

        select rev_name, mov_title, rev_stars from movie_rating mr 
        join movie m on m.mov_id = mr.mov_id 
        join movie_reviewer mer on mr.rev_id = mer.rev_id;

        +--------------------+---------------------+-----------+
        | rev_name           | mov_title           | rev_stars |
        +--------------------+---------------------+-----------+
        | Righty Sock        | Vertigo             |       8.4 |
        | Jack Malvern       | The Innocents       |       7.9 |
        | Flagrant Baronessa | Lawrence of Arabia  |       8.3 |
        | NULL               | Blade Runner        |       8.2 |
        | Victor Woeltjen    | Avatar              |       7.3 |
        | Simon Wright       | The Usual Suspects  |       8.6 |
        | Neal Wruck         | Chinatown           |      NULL |
        | Paul Monks         | Boogie Nights       |       3.0 |
        | Mike Salvati       | Annie Hall          |       8.1 |
        | NULL               | Princess Mononoke   |       8.4 |
        | Sasha Goldshtein   | American Beauty     |       7.0 |
        | Righty Sock        | Titanic             |       7.7 |
        | Josh Cates         | Good Will Hunting   |       4.0 |
        | Krug Stillo        | Seven Samurai       |       7.7 |
        | Scott LeBrun       | Trainspotting       |      NULL |
        | Hannah Steele      | Donnie Darko        |       8.1 |
        | Vincent Cadena     | Slumdog Millionaire |       8.0 |
        | Brandt Sponseller  | Aliens              |       8.4 |
        | Richard Adams      | Beyond the Sea      |       6.7 |
        +--------------------+---------------------+-----------+


	40. Write a SQL query to find those movies that have at least one rating and received the most stars. Sort the result-set on movie title. Return movie title and maximum review stars.

        select mov_title from movie 
        where mov_id = (select mov_id from movie_rating where rev_stars = ((select max(rev_stars) from movie_rating where num_o_ratings > 1))) 
        ORDER BY mov_title;


	41. Write a SQL query to find out which movies have received ratings. Return movie title, director first name, director last name and review stars.

        select d.dir_fname, d.dir_lname, m.mov_title, mr.num_o_ratings from movie_direction md 
        join director d on md.dir_id = d.dir_id 
        join movie m on md.mov_id = m.mov_id 
        join movie_rating mr on md.mov_id = mr.mov_id;

        +-----------+-----------------+---------------------+---------------+
        | dir_fname | dir_lname       | mov_title           | num_o_ratings |
        +-----------+-----------------+---------------------+---------------+
        | Alfred    | Hitchcock       | Vertigo             |        263575 |
        | Jack      | Clayton         | The Innocents       |         20207 |
        | David     | Lean            | Lawrence of Arabia  |        202778 |
        | Ridley    | Scott           | Blade Runner        |        484746 |
        | Bryan     | Singer          | The Usual Suspects  |        779489 |
        | Roman     | Polanski        | Chinatown           |        227235 |
        | Paul      | Thomas Anderson | Boogie Nights       |        195961 |
        | Woody     | Allen           | Annie Hall          |        203875 |
        | Hayao     | Miyazaki        | Princess Mononoke   |          NULL |
        | Sam       | Mendes          | American Beauty     |        862618 |
        | James     | Cameron         | Titanic             |        830095 |
        | Gus       | Van Sant        | Good Will Hunting   |        642132 |
        | Danny     | Boyle           | Trainspotting       |        580301 |
        | Richard   | Kelly           | Donnie Darko        |        609451 |
        | Danny     | Boyle           | Slumdog Millionaire |        667758 |
        | James     | Cameron         | Aliens              |        511613 |
        | Kevin     | Spacey          | Beyond the Sea      |         13091 |
        +-----------+-----------------+---------------------+---------------+

	42. Write a SQL query to find movies in which one or more actors have acted in more than one film. Return movie title, actor first and last name, and the role.

        select m.mov_title, a.act_fname, a.acrt_lname, mc.role from movie_cast mc 
        join actor a on mc.act_id = a.act_id 
        join movie m on mc.mov_id = m.mov_id where mc.mov_id = (select mov_id from movie_cast group by mov_id having count(*) > 1);

        +-----------+-----------+------------+---------------+
        | mov_title | act_fname | acrt_lname | role          |
        +-----------+-----------+------------+---------------+
        | Chinatown | Jack      | Nicholson  | J.J. Gittes   |
        | Chinatown | Christian | Bale       | Alfred Borden |
        +-----------+-----------+------------+---------------+


	43. Write a SQL query to find the actor whose first name is 'Claire' and last name is 'Danes'. Return director first name, last name, movie title, actor first name and last name, role.

        select d.dir_fname, d.dir_lname, m.mov_title, a.act_fname, a.acrt_lname from movie_cast mc 
        join movie m on mc.mov_id = m.mov_id 
        join actor a on mc.act_id = a.act_id
        join movie_direction md on mc.mov_id = md.mov_id
        join director d on md.dir_id = d.dir_id
        where mc.act_id = (select a.act_id from actor a where act_fname = 'Claire' and acrt_lname = 'Danes');

        +-----------+-----------+-------------------+-----------+------------+
        | dir_fname | dir_lname | mov_title         | act_fname | acrt_lname |
        +-----------+-----------+-------------------+-----------+------------+
        | Hayao     | Miyazaki  | Princess Mononoke | Claire    | Danes      |
        +-----------+-----------+-------------------+-----------+------------+


	44. Write a SQL query to find for actors whose films have been directed by them. Return actor first name, last name, movie title and role.

        select a.act_fname, a.acrt_lname, m.mov_title, mc.role from movie_cast mc
        join movie m on mc.mov_id = m.mov_id 
        join movie_direction md on mc.mov_id = md.mov_id
        join actor a on mc.act_id = a.act_id 
        join director d on md.dir_id = d.dir_id
        where a.act_fname = d.dir_fname and acrt_lname = d.dir_lname;

        +-----------+------------+----------------+-------------+
        | act_fname | acrt_lname | mov_title      | role        |
        +-----------+------------+----------------+-------------+
        | Woody     | Allen      | Annie Hall     | Alvy Singer |
        | Kevin     | Spacey     | Beyond the Sea | Bobby Darin |
        +-----------+------------+----------------+-------------+

	45. Write a SQL query to find the cast list of the movie ‘Chinatown’. Return first name, last name.

        select a.act_fname, a.acrt_lname from movie m
        join movie_cast mc on m.mov_id = mc.mov_id 
        join actor a on mc.act_id = a.act_id
        where mov_title = 'Chinatown';

        +-----------+------------+
        | act_fname | acrt_lname |
        +-----------+------------+
        | Jack      | Nicholson  |
        | Christian | Bale       |
        +-----------+------------+

	46. Write a SQL query to find those movies where actor’s first name is 'Harrison' and last name is 'Ford'. Return movie title.

        select m.mov_title from actor a
        join movie_cast mc on a.act_id = mc.act_id
        join movie m on mc.mov_id = m.mov_id
        where a.act_fname = 'Harrison' and a.acrt_lname = 'Ford';
        
        +--------------+
        | mov_title    |
        +--------------+
        | Blade Runner |
        +--------------+

	47. Write a SQL query to find the highest-rated movies. Return movie title, movie year, review stars and releasing country.

        select m.mov_title from movie m 
        join movie_rating mr on m.mov_id = mr.mov_id
        where mr.num_o_ratings = 
        (select max(num_o_ratings) from movie_rating);

        +-----------------+
        | mov_title       |
        +-----------------+
        | American Beauty |
        +-----------------+


	48. Write a SQL query to find the highest-rated ‘Mystery Movies’. Return the title, year, and rating.

        select m.mov_title, m.mov_year, mr.rev_stars AS rating
        from movie m
        join movie_rating mr on m.mov_id = mr.mov_id
        join movie_genres mg on m.mov_id = mg.mov_id
        join genres g  on mg.gen_id = g.gen_id
        where g.gen_title = 'Mystery'
        AND mr.rev_stars = (
                select MAX(mr2.rev_stars)
                from movie_rating mr2
                join movie_genres mg2 on mr2.mov_id = mg2.mov_id
                join genres g2 on mg2.gen_id = g2.gen_id
                where g2.gen_title = 'Mystery'
        );

        +-----------+----------+--------+
        | mov_title | mov_year | rating |
        +-----------+----------+--------+
        | Vertigo   |     1958 |    8.4 |
        +-----------+----------+--------+


	49. Write a SQL query to find the years when most of the ‘Mystery Movies’ produced. Count the number of generic title and compute their average rating. Group the result set on movie release year, generic title. Return movie year, generic title, number of generic title and average rating.

        select m.mov_year AS movie_year, g.gen_title AS generic_title, COUNT(m.mov_id) AS movie_count, AVG(mr.rev_stars) AS avg_rating
        from movie m
        join movie_genres mg ON m.mov_id = mg.mov_id
        join genres g ON mg.gen_id = g.gen_id
        join movie_rating mr ON m.mov_id = mr.mov_id
        where g.gen_title = 'Mystery'
        group BY m.mov_year, g.gen_title
        order BY movie_count DESC, m.mov_year;

        +------------+---------------+-------------+------------+
        | movie_year | generic_title | movie_count | avg_rating |
        +------------+---------------+-------------+------------+
        |       1958 | Mystery       |           1 |    8.40000 |
        +------------+---------------+-------------+------------+

	50. Write a query in SQL to generate a report, which contain the fields movie title, name of the female actor, year of the movie, role, movie genres, the director, date of release, and rating of that movie.

        select m.mov_title AS movie_title, CONCAT(a.act_fname, ' ', a.acrt_lname) AS female_actor, m.mov_year AS movie_year, mc.role, g.gen_title AS movie_genre, CONCAT(d.dir_fname, ' ', d.dir_lname) AS director, m.mov_dt_rel AS release_date,
        mr.rev_stars AS rating
        from movie m
        join movie_cast mc on m.mov_id = mc.mov_id
        join actor a on mc.act_id = a.act_id
        join movie_genres mg on m.mov_id = mg.mov_id
        join genres g on mg.gen_id = g.gen_id
        join movie_direction md on m.mov_id = md.mov_id
        join director d on md.dir_id = d.dir_id
        left join movie_rating mr on m.mov_id = mr.mov_id
        where a.act_gender = 'F'
        order by m.mov_year, m.mov_title;

    +-------------------+------------------+------------+---------------+-------------+-----------------+--------------+--------+
    | movie_title       | female_actor     | movie_year | role          | movie_genre | director        | release_date | rating |
    +-------------------+------------------+------------+---------------+-------------+-----------------+--------------+--------+
    | The Innocents     | Deborah Kerr     |       1961 | Miss Giddens  | Horror      | Jack Clayton    | 1962-02-19   |    7.9 |
    | Aliens            | Sigourney Weaver |       1986 | Ripley        | Action      | James Cameron   | 1986-08-29   |    8.4 |
    | Princess Mononoke | Claire Danes     |       1997 | San           | Animation   | Hayao Miyazaki  | 2001-10-19   |    8.4 |
    | Eyes Wide Shut    | Nicole Kidman    |       1999 | Alice Harford | Mystery     | Stanley Kubrick | NULL         |   NULL |
    +-------------------+------------------+------------+---------------+-------------+-----------------+--------------+--------+


