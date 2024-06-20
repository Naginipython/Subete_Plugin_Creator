function getChapters(json, html) {
  html = JSON.parse(html); 
  json.chapters = html['data'].map(e => {
    return {
      number: parseFloat(e['attributes']['chapter'])? parseFloat(e['attributes']['chapter']) : 0.0,
      id: e['id'],
      title: e['attributes']['title'] == '' || e['attributes']['title'] == null? `Chapter ${e['attributes']['chapter']}` : e['attributes']['title'],
      page: 1,
      completed: false
    }
  });
  return json;
}