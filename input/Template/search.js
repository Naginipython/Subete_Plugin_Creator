// NOTE: NO COMMENTS. BREAKS CODE. DELETE THESE ONCE FINISHED
// Takes in HTML, without \n, whitespace, and discarded some ascii
// returns:
// {
//   id: String,
//   title: String,
//   img: String,
//   plugin: String,
//   authors: String,
//   artists: String,
//   description: String OR none,
//   chapters: []
// }
function search(html) {
    let data = [];
    const regex = new RegExp(`[your regex here](.*?)[ending regex]`, 'gi');
    let match;
    while ((match = regex.exec(html)) !== null) {
      let lib_item = {};
      lib_item.id = match[1];
      lib_item.title = '';
      lib_item.img = '';
      lib_item.plugin = 'MangaRaw';
      lib_item.authors = '';
      lib_item.artists = '';
      lib_item.description = '';
      lib_item.chapters = [];
      data.push(lib_item);
    }
    return data;
}