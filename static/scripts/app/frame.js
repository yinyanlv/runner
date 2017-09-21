$(function () {
  var frame = {
    init: function () {
      var self = this;

      self.initElements();
      self.initEvents();
    },

    initElements: function () {
      var self = this;

      self.$mBtnMenu = $('#m-btn-menu');
      self.$mNav = $('#m-nav');
      self.$mNavBg = $('#m-nav-bg');
      self.$backToTop = $('.back-to-top');
      self.$btnSelectFiles = $('#btn-select-files');
      self.$inputFiles = $('#input-files');
    },

    initEvents: function() {
      var self = this;

      self.$mBtnMenu.on('click', function () {

        if (self.$mNav.is(':visible')) {

          self.$mNav.hide();
          self.$mNavBg.hide();
        } else {

          self.$mNav.show();
          self.$mNavBg.show();
        }
      });

      self.$mNavBg.on('click', function () {

        self.$mNav.hide();
        self.$mNavBg.hide();
      });

      self.$backToTop.on('click', function () {

        $('html, body').animate({
          scrollTop: 0
        }, 300);
      });

      self.$btnSelectFiles.on('click', function () {

        self.$inputFiles.click();
      });

      self.$inputFiles.on('change', function (e) {

        var files = e.target.files;

        for (var i = 0; i < files.length; i++) {

          console.log(files[i].name);
        }
      });
    }
  };

  frame.init();
});