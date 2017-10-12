$(function () {

    var reset = {

        init: function () {
            var self = this;

            self.initElements();
            self.initPlugins();
            self.initEvents();
        },

        initElements: function () {
            var self = this;

            self.$btnResetPassword = $('#btn-reset-password');
            self.$inputUsername = $('#username');
        },

        initPlugins: function () {
            var self = this;

            self.validator = Validator ? new Validator({
                form: '#form-reset-password',
                submit: self.reset.bind(this)
            }) : null;
        },

        initEvents: function () {
            var self = this;

            self.$btnResetPassword.on('click', function () {

                self.reset();
            });
        },

        reset: function () {
            var self = this;

            if (!self.validator.isValid()) return;
            if (self.$btnResetPassword.is('.disabled')) return;

            self.$btnResetPassword.addClass('disabled');

            var params = self.validator.getValues();

            params.username = $.trim(self.$inputUsername.val());

            $.ajax({
                url: globalConfig.path + '/set-new-password',
                type: 'POST',
                data: params,
                success: function (res) {

                    if (res.success) {

                        alert(res.message);
                        window.location.href = res.data;
                    } else {

                        self.validator.showError(null, res.message);
                    }
                },
                complete: function () {
                    self.$btnResetPassword.removeClass('disabled');
                }
            });
        }
    };

    reset.init();
});