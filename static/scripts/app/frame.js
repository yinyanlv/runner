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
    }
  };

  frame.init();
});