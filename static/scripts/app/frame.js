$(function () {
    var frame = {
        init: function () {
            var self = this;

            self.initElements();
            self.initPlugins();
            self.initEvents();
        },

        initElements: function () {
            var self = this;

            self.$mBtnMenu = $('#m-btn-menu');
            self.$mNav = $('#m-nav');
            self.$mNavBg = $('#m-nav-bg');
            self.$backToTop = $('.back-to-top');
        },

        initPlugins: function () {
            var self = this;
            var $datetimeAgoList = $('.datetime-ago');

            for (var i = 0; i < $datetimeAgoList.length; i++) {

                var $datetimeAgo = $($datetimeAgoList[i]);
                var datetime = $datetimeAgo.data('datetime');

                $datetimeAgo.html(moment && moment(datetime).fromNow());
            }
        },

        initEvents: function () {
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
        }
    };

    frame.init();
});
