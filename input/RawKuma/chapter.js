function getChapters(json, html) {
  const auth_regex = new RegExp(`<b>Author</b> <span> (.*?) </span>`, 'gi');
  json.authors = auth_regex.exec(html)[1];
  const art_regex = new RegExp(`<b>Artist</b> <span> (.*?) </span>`, 'gi');
  json.artists = art_regex.exec(html)[1];
  const desc_regex = new RegExp(`itemprop="description"><p>(.*?)</p>`, 'gi');
  let desc_match = desc_regex.exec(html);
  if (desc_match != null) {
    json.description = desc_match[1];
  } else {
    const desc_regex2 = new RegExp(`<hr /> <p>(.*?)</p>`, 'gi');
    json.description = desc_regex2.exec(html)[1];
  }
  const regex = new RegExp(`<div class="eph-num"> <a href="https://rawkuma.com/(.*?)/"> <span class="chapternum">(.*?)</span>`, 'gi');
  let data = [];
  let match;
  while ((match = regex.exec(html)) !== null) {
    let lib_item = {};
    lib_item.id = match[1];
    lib_item.title = '';
    let num_match = match[2].match(new RegExp(`\\d+`));
    lib_item.number = num_match != null? parseInt(num_match[0]) : 0;
    lib_item.page = 1;
    lib_item.completed = false;
    data.push(lib_item);
  }
  json.chapters = data;
  return json;
}