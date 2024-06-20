function search(html) {
    let data = [];
    const regex = new RegExp(`<div class="row c-tabs-item__content">(.*?)</span></div></div></div></div></div>`, 'gi');
    let data_to_parse = [];
    let match;
    while ((match = regex.exec(html)) !== null) {
      data_to_parse.push(match[1]);
    }
    for (let item of data_to_parse) {
      let lib_item = {};
      const id_regex = new RegExp(`<h3 class="h4"><a href="https://asuralightnovel.com/novel/(.*?)/">(.*?)</a>`, 'gi');
      let reg = id_regex.exec(item);
      lib_item.id = reg[1];
      lib_item.title = reg[2];
      const img_regex = new RegExp(`data-src="(.*?)"`, 'gi');
      lib_item.img = img_regex.exec(item)[1];
      const author_regex = new RegExp(`<a href="https://asuralightnovel.com/novel-author/.*?/">(.*?)</a>`, 'gi');
      lib_item.authors = author_regex.exec(item)[1];
      const artist_regex = new RegExp(`<a href="https://asuralightnovel.com/novel-artist/.*?/">(.*?)</a>`, 'gi');
      lib_item.artists = artist_regex.exec(item)[1];
      lib_item.description = '';
      lib_item.plugin = "Asura Light Novel";
      lib_item.chapters = [];
      data.push(lib_item);
    }
    return data;
}