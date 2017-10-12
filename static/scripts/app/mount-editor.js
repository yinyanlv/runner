(function (Editor, markdownit) {

    // Set default options
    var md = new markdownit();
    var toolbar = Editor.toolbar;

    md.set({
        html: false,  // Enable HTML tags in source
        xhtmlOut: false,  // Use '/' to close single tags (<br />)
        breaks: true,  // Convert '\n' in paragraphs into <br>
        langPrefix: 'lang-',  // CSS language prefix for fenced blocks
        linkify: false,  // Autoconvert URL-like text to links
        typographer: false  // Enable smartypants and other sweet transforms
    });

    window.markdowniter = md;

    // 追加内容
    Editor.prototype.push = function (txt) {
        var cm = this.codemirror;
        var line = cm.lastLine();
        cm.setLine(line, cm.getLine(line) + txt);
    };

    var replaceTool = function (name, callback) {
        for (var i = 0, len = toolbar.length; i < len; i++) {
            var v = toolbar[i];
            if (typeof(v) !== 'string' && v.name === name) {
                v.action = callback;
                break;
            }
        }
    };

    replaceTool('image', function (editor) {

        if (Uploader) {
            var uploader = new Uploader({
                success: function (res) {
                    var result = res[0];
                    var str  = '';

                    for (var i = 0; i < result.data.length; i++) {
                        var item = result.data[i];

                        str += '![' + item.filename + '](' + globalConfig.uploadPath + '/' + item.path + ') ';
                    }
                    editor.push(str);
                },
                error: function () {

                    console.log('upload error');
                }
            });

            uploader.show();
        }
    });

})(window.Editor, window.markdownit);
