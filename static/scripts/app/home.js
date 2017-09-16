$(function () {
  var app = {
    init: function () {
      var self = this;

      self.initElements();
      self.initEvents();
    },

    initElements: function () {
      var self = this;

      self.$mBtnMenu = $('#m-btn-menu');
      self.$mNav = $('#m-nav');

    },

    initEvents: function() {
      var self = this;

      self.$mBtnMenu.on('click', function () {

        self.$mNav.is(':visible') ? self.$mNav.hide() : self.$mNav.show();
      });
    }
  };

  app.init();
});