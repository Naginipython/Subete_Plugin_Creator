function getChapters(json, html) {
  let retrieved = JSON.parse(html.match(new RegExp(`vm.Chapters = (.*?);`))[1]);
  json.chapters = retrieved.map(e => {
    let decimal = parseFloat('0.'+e['Chapter'][5]);
    let num = parseInt(e['Chapter'].slice(1, 5));
    return {
       'id': `${json.id}-chapter-${num+decimal}`,
       'number': num+decimal,
       'title': e['ChapterName'] == null? '' : e['ChapterName'],
       'page': 1,
       'completed': false
    }
  });
  return json;
}