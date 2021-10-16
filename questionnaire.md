1. *Are there any sub-optimal choices( or short cuts taken due to limited time ) in your implementation?*
> The CSS isn't great, the same look could be achieved in a much smaller file. Also some things aren't aligned perfectly, and this page wouldn't look great on devices of different sizes. I also didn't use a backend framework like node or django, I knew the backend could be pretty basic so I didn't think the added complexity was necessary. I also think the file structure could be redone, it's a bit messy looking because python's `http.server` will serve the current directory.

2. *Is any part of it over-designed? ( It is fine to over-design to showcase your skills as long as you are clear about it)*
> It's pretty simple, we're not even watching some api endpoint on the server, it's just hosting the JSON file. Also, the script that fetches data from the exchanges just overwrites any past data, so we don't really worry about persistence. The frontend isn't that complicated since its only job is to read the data from the server, determine the recommendations, and display the data.

3. *If you have to scale your solution to 100 users/second traffic what changes would you make, if any?*
> I believe this implementation can already scale to 100 users/second. The server is extremely basic, we just host a JSON file that *might* update every 4 seconds. The JSON file is likely to be under 400 bytes, and the clients make extremely minimal `GET` requests, so we don't really send a lot of data.

> Here's a [script](stress_test.py) that can test the traffic bandwidth. If I wanted to scale it beyond what it can currently handle, I would probably use a better webserver, maybe use a backend framework like Django, certainly implement some form of caching (so the server doesn't send data the client already has), upgrade the hardware, or even use faster programming language (Java/Scala, Go, Rust).
4. *What are some other enhancements you would have made, if you had more time to do this implementation*
> I would add a very simple animation to show when prices or recommendations change (ex. change opacity, or italicize briefly), as it can be difficult to tell with such barebones text.
> 
> I'd also add change the `buy/sell` button links to take the user to the actual market where they can start trading. I don't have an account with coinbase or kraken, so I wasn't entirely sure where to find those links on their web apps.
> 
> If I had more time, I would like to try using websockets to get a live feed of data rather than having the client ask the server for new data. If I were to keep the current implementation (http requests), I would at least add some form of caching so that the server wouldn't send the client data that it already had.
