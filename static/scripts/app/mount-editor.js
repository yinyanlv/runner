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
        success: function () {

          console.log('upload success');
        },
        error: function () {

          console.log('upload error');
        }
      });

      uploader.show();
    }
  });

  function _replaceSelection(cm, active, start, end) {
    var text;
    var startPoint = cm.getCursor('start');
    var endPoint = cm.getCursor('end');
    var end = end || '';

    if (active) {
      text = cm.getLine(startPoint.line);
      start = text.slice(0, startPoint.ch);
      end = text.slice(startPoint.ch);
      cm.setLine(startPoint.line, start + end);
    } else {
      text = cm.getSelection();
      cm.replaceSelection(start + text + end);

      startPoint.ch += start.length;
      endPoint.ch += start.length;
    }
    cm.setSelection(startPoint, endPoint);
    cm.focus();
  }

  /**
   * The state of CodeMirror at the given position.
   */
  function getState(cm, pos) {

    pos = pos || cm.getCursor('start');

    var stat = cm.getTokenAt(pos);
    if (!stat.type) return {};

    var types = stat.type.split(' ');
    var ret = {}, data, text;

    for (var i = 0; i < types.length; i++) {
      data = types[i];
      if (data === 'strong') {
        ret.bold = true;
      } else if (data === 'variable-2') {
        text = cm.getLine(pos.line);
        if (/^\s*\d+\.\s/.test(text)) {
          ret['ordered-list'] = true;
        } else {
          ret['unordered-list'] = true;
        }
      } else if (data === 'atom') {
        ret.quote = true;
      } else if (data === 'em') {
        ret.italic = true;
      }
    }
    return ret;
  }

})(window.Editor, window.markdownit);
