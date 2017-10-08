$(function () {
    var frame = {
        init: function () {
            var self = this;

            self.initElements();
            self.initPlugins();
            self.initEvents();

            self.checkBackToTop();
        },

        initElements: function () {
            var self = this;

            self.$window = $(window);
            self.$mBtnMenu = $('#m-btn-menu');
            self.$mNav = $('#m-nav');
            self.$mNavBg = $('#m-nav-bg');
            self.$backToTop = $('.back-to-top');
            self.$inputSearchList = $('.input-search');
            self.$btnSearchList = $('.btn-search');
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

            self.$window.scroll(function() {

                self.checkBackToTop();
            });

            self.$inputSearchList.on('keydown', function (e) {

                if (e.keyCode === 13) {
                    e.preventDefault();
                    if (self.validSearch(this)) {
                        $(this).closest('.search').submit();
                    }
                }
            });

            self.$btnSearchList.on('click', function () {

                if (self.validSearch(this)) {
                    $(this).closest('.search').submit();
                }
            });
        },

        checkBackToTop: function () {
            var self = this;

            if (self.$window.scrollTop() > 200) {

                self.$backToTop.fadeIn();

            } else {
                self.$backToTop.fadeOut();
            }
        },

        validSearch: function (that) {
            var self = this;
            var $input = $(that).closest('.search').find('.input-search');

            if ($input.val().length === 0) {

                alert('关键字不可为空！');
                return false;
            } else {

                return true;
            }
        }
    };

    frame.init();
});
