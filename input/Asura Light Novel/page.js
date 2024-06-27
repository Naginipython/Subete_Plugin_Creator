function getChapterPages(html) {
    let data = [];
    const regex = new RegExp(`<p>(.*?)</p>`, 'gi');
    let match;
    while ((match = regex.exec(html)) !== null) {
      if (!match[1].includes(`<`) && match[1] != '' && match[1] != "© 2021 Asura Light Novel Inc. All rights reserved") {
        data.push(match[1]);
      }
    }
    return data;
  }