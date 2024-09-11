function search(html) {
    let data = [];
    const narrowRegex = new RegExp(`<ul class="items">(.*?)<ul>`, 'gi');
    html = narrowRegex.exec(html)[1];
    const regex = new RegExp(`<li>(.*?)</li>`, 'gi');
    let match;
    while ((match = regex.exec(html)) !== null) {
        let lib_item = {};
        const idRegex = new RegExp(`/category/(.*?)"`, 'gi');
        lib_item.id = idRegex.exec(match)[1];
        const titleRegex = new RegExp(`title="(.*?)"`);
        lib_item.title = titleRegex.exec(match)[1];
        const imgRegex = new RegExp(`src="(.*?)"`, 'gi');
        lib_item.img = imgRegex.exec(match)[1];
        lib_item.plugin = 'Gogoanime';
        lib_item.studio = '';
        lib_item.status = '';
        lib_item.description = '';
        lib_item.episodes = [];
        data.push(lib_item);
    }
    return data;
}