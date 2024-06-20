// NOTE: NO COMMENTS. BREAKS CODE. DELETE THESE ONCE FINISHED
// Takes in JSON from search and HTML, without \n, whitespace, and discarded some ascii
// returns:
// {
//   id: String,
//   title: String,
//   img: String,
//   plugin: String,
//   authors: String,
//   artists: String,
//   description: String OR none,
//   chapters: [
//       {
//           id: String,
//           number: Float,
//           title: String,
//           page: Number,
//           completed: Boolean
//       }
//   ]
// }
function getChapters(json, html) {
  let data = [];
  const regex = new RegExp(`[your regex here](.*?)[ending regex]`, 'gi');
  let match;
  while ((match = regex.exec(html)) !== null) {
    let lib_item = {};
    lib_item.id = match[1];
    lib_item.title = '';
    lib_item.number = -1;
    lib_item.page = 1;
    lib_item.completed = false;
    data.push(lib_item);
  }
  json.chapters = data;
  return json;
}