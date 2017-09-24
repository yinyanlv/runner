$(function () {

    var login = {

        init: function () {
            var self = this;

            self.initElements();
            self.initPlugins();
            self.initEvents();
        },

        initElements: function () {
            var self = this;

            self.$btnLogin = $('#btn-login');
        },

        initPlugins: function () {
            var self = this;

            self.validator = Validator ? new Validator({
                form: '#form-login',
                submit: self.login.bind(this)
            }) : null;
        },

        initEvents: function () {
            var self = this;

            self.$btnLogin.on('click', function () {

                self.login();
            });
        },

        login: function () {
            var self = this;

            if (!self.validator.isValid()) return;
            if (self.$btnLogin.is('disabled')) return;

            self.$btnLogin.addClass('disabled');

            var params = self.validator.getValues();

            $.ajax({
                url: globalConfig.path + '/login',
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
                    self.$btnLogin.removeClass('disabled');
                }
            });
        }
    };

    login.init();
});