function getChapterPages(html) {
  let retrieved = JSON.parse(html.match(new RegExp(`vm.CurChapter = (.*?);`))[1]);
  let link = JSON.parse(html.match(new RegExp(`vm.CurPathName = (.*?);`))[1]);
  let id = JSON.parse(html.match(new RegExp(`vm.IndexName = (.*?);`))[1]);
  
  let chapter = retrieved.Chapter.slice(1,-1);
  let period = retrieved.Chapter[retrieved.Chapter.length -1];
  let pages = parseInt(retrieved.Page);
  let data = [];
  for (let i=1; i < pages+1; i++) {
    if (period != 0) {
        let newChap = chapter + '.' + period;
        let pad_math = 5 + newChap.split('.')[1].length; 
        data.push(`https://${link}/manga/${id}/${newChap.padStart(pad_math, '0')}-${i.toString().padStart(3, '0')}.png`)
    } else {
        data.push(`https://${link}/manga/${id}/${chapter.padStart(4, '0')}-${i.toString().padStart(3, '0')}.png`)
    }
  }
  return data;
}