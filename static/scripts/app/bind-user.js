$(function () {

    var bind = {

        init: function () {
            var self = this;

            self.initElements();
            self.initPlugins();
            self.initEvents();
        },

        initElements: function () {
            var self = this;

            self.$btnBind = $('#btn-bind');
            self.$inputUserInfo = $('#user-info');
        },

        initPlugins: function () {
            var self = this;

            self.validator = Validator ? new Validator({
                form: '#form-bind',
                submit: self.bind.bind(this)
            }) : null;
        },

        initEvents: function () {
            var self = this;

            self.$btnBind.on('click', function () {

                self.bind();
            });
        },

        bind: function () {
            var self = this;

            if (!self.validator.isValid()) return;
            if (self.$btnBind.is('.disabled')) return;

            self.$btnBind.addClass('disabled');

            var params = self.validator.getValues();

            params.userInfo = $.trim(self.$inputUserInfo.val());

            $.ajax({
                url: globalConfig.path + '/bind-user',
                type: 'POST',
                data: params,
                success: function (res) {

                    if (res.success) {

                        window.location.href = res.data;
                    } else {

                        self.validator.showError(null, res.message);
                    }
                },
                complete: function () {
                    self.$btnBind.removeClass('disabled');
                }
            });
        }
    };

    bind.init();
});