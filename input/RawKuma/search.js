function search(html) {
    let data = [];
    const regex = new RegExp(`<div class="bs"> (.*?)href="https://rawkuma.com/manga/(.*?)/" title="(.*?)"> (.*?)src="(.*?)"`, 'gi');
    let match;
    while ((match = regex.exec(html)) !== null) {
      let lib_item = {};
      lib_item.id = match[2];
      lib_item.title = match[3];
      lib_item.img = match[5];
      lib_item.plugin = 'RawKuma';
      lib_item.authors = '';
      lib_item.artists = '';
      lib_item.description = '';
      lib_item.chapters = [];
      data.push(lib_item);
    }
    return data;
}