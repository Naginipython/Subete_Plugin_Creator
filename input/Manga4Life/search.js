function search(html) {
  console.log("here");
  let retrieved = JSON.parse(html.match(new RegExp(`vm.Directory = (.*?); vm`))[1]);
  let query = JSON.parse(html.match(new RegExp(`vm.Search = (.*?);`))[1]).SeriesName;
  let filtered_retrieve = retrieved.filter(e => 
    e['s'].toLowerCase().includes(query) || 
    e['al'].some(f => f.toLowerCase().includes(query)) ||
    e['a'].some(f => f.toLowerCase().includes(query))
  );
  return filtered_retrieve.map(e => {
    return {
      'id': e['i'],
      'title': e['s'],
      'img': `https://temp.compsci88.com/cover/${e['i']}.jpg`,
      'plugin': 'Manga4Life',
      'authors': e['a'].join(', '),
      'artists': '',
      'description': '',
      'chapters': []
    };
  });
}