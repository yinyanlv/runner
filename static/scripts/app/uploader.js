$(function () {

  function Uploader(options) {

    var mainTemplate =
        '<div class="dialog-bg" id="dialog-bg"></div>\
        <div class="dialog-wrapper" id="dialog-wrapper">\
          <div class="dialog-header">上传图片<span class="btn" id="btn-close-dialog"><i class="fa fa-close"></i></span></div>\
          <div class="dialog-content">\
            <form id="form-upload">\
              <input type="file" name="files" id="input-files" multiple>\
            </form>\
            <div class="btn-line">\
              <span class="btn btn-primary" id="btn-select-files">选择文件</span>\
              <span class="btn btn-primary" id="btn-upload">上传</span>\
            </div>\
            <div class="file-list" id="file-list">\
            </div>\
          </div>\
        </div>';

    var fileTemplate = '<div class="file" data-index="${index}"><div class="filename">${filename}</div><span class="btn btn-delete-file"><i class="fa fa-trash"></i></span></div>';

    return {
      show: function () {
        var self = this;

        self.init();

        $('body').append(mainTemplate);

        self.initElements();
        self.initEvents();
      },

      hide: function () {
        var self = this;

        self.$dialogBg.remove();
        self.$dialogWrapper.remove();
      },

      init: function () {
        var self = this;

        self.files = [];

      },

      initElements: function () {
        var self = this;

        self.$dialogBg = $('#dialog-bg');
        self.$dialogWrapper= $('#dialog-wrapper');
        self.$btnCloseDialog = $('#btn-close-dialog');
        self.$btnSelectFiles = $('#btn-select-files');
        self.$btnUpload = $('#btn-upload');
        self.$formUpload = $('#form-upload');
        self.$inputFiles = $('#input-files');
        self.$fileList = $('#file-list');
      },

      initEvents: function () {
        var self = this;

        self.$btnCloseDialog.on('click', function () {

          self.hide();
        });

        self.$btnSelectFiles.on('click', function () {

          self.$inputFiles.click();
        });

        self.$inputFiles.on('change', function (e) {

          self.files = Array.prototype.slice.call(e.target.files);

          self.renderFileList();
        });

        self.$btnUpload.on('click', function () {

          if (!self.files.length) return alert('当前未选中任何图片文件');

          if (!self.checkFileType()) return alert('只能上传图片文件');

          var formData = new FormData();

          for (var i = 0; i < self.files.length; i++) {

            formData.append('file' + i, self.files[i]);
          }

          $.ajax({
            url: '/upload',
            type: 'POST',
            data: formData,
            cache: false,
            contentType: false,
            processData: false,  // 是否需要序列化data
            success: function () {

              self.hide();
              options && options.success && options.success(arguments);
            },
            error: function () {

              options && options.error && options.error(arguments);
            }
          });
        });

        self.$dialogWrapper.on('click', function (e) {
          var $target = $(e.target);

          if ($target.closest('.btn-delete-file').length) {

            var $file = $target.closest('.file');
            var index = $file.data('index');

            $file.remove();

            self.files && self.files.splice(index, 1);

            self.renderFileList();
          }
        });
      },

      renderFileList: function () {
        var self = this;
        var str = '';

        for (var i = 0; i < self.files.length; i++) {

          str += fileTemplate
                    .replace(/\$\{filename\}/g, self.files[i].name)
                    .replace(/\$\{index\}/g, i);
        }

        self.$fileList.html(str);
      },

      checkFileType: function () {
        var self = this;
        var isValid = true;

        for (var i = 0; i < self.files.length; i++) {

          var type = self.files[i].type.split('/')[0];

          if (type !== 'image') {

            isValid = false;
            break;
          }
        }

        return isValid;
      }
    };
  }

  window['Uploader'] = Uploader;
});



