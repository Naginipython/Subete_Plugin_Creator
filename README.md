# Subete_Plugin_Creator

Used to create plugins for the Subete app. To use this, follow details within the template, located `input/template/`. Here, put links into `.txt` files and create JavaScript scrapers that can be used to get the data needed for Subete to function. \
Create a folder for your plugin in the `input` folder with your desired plugin name, and try to follow the templates. For debugging, and to complete the process, run `cargo run [Folder name]`. [Folder name] does not have to include `input`.

### URLS
Located in `search.txt`, `chapter.txt` and `page.txt`. `search.txt` will require `{title}` at some point in the URL, to be replace by a user's query, and `chapter.txt` and `page.txt` will require `{id}` to get specific page data. `id` is a general term, so here's an example:\
Website: Manga4Life\
Desired manga info: `One Piece`\
Link to desired manga info: https://manga4life.com/manga/One-Piece\
Manga id: One-Piece
Link to desired chapter: https://manga4life.com/read-online/One-Piece-chapter-1118-page-1.html\
Chapter id: `One-Piece-chapter-1118`\
Note: The exact page is set within the `page.js` scraper