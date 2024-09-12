function getChapters(json, html) {
  json = JSON.parse(json);
  let data = [];
  const regex = new RegExp(`<li class="wp-manga-chapter "> (.*?) </li>`, 'gi');
  let data_to_parse = [];
  let match;
  while ((match = regex.exec(html)) !== null) {
    data_to_parse.push(match[1]);
  }
  for (let item of data_to_parse) {
    let lib_item = {};
    const id_regex = new RegExp(`<a href="https://asuralightnovel.com/novel/(.*?)/(.*?)/"> (.*?) </a>`, 'gi');
    let reg = id_regex.exec(item);
    lib_item.id = reg[1] + "/" + reg[2];
    lib_item.title = reg[3];
    lib_item.number = parseInt(lib_item.title.match(new RegExp(`\\d+`))[0]);
    lib_item.page = 1;
    lib_item.completed = false;
    data.push(lib_item);
  }
  json.chapters = data;
  return json;
}