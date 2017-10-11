$(function () {

    var register = {

        init: function () {
            var self = this;

            self.initElements();
            self.initPlugins();
            self.initEvents();
        },

        initElements: function () {
            var self = this;

            self.$btnRegister = $('#btn-register');
        },

        initPlugins: function () {
            var self = this;

            self.validator = Validator ? new Validator({
                form: '#form-register',
                submit: self.register.bind(this)
            }) : null;
        },

        initEvents: function () {
            var self = this;

            self.$btnRegister.on('focus click', function () {

                self.register();
            });
        },

        register: function () {
            var self = this;

            if (!self.validator.isValid()) return;
            if (self.$btnRegister.is('.disabled')) return;

            self.$btnRegister.addClass('disabled');

            var params = self.validator.getValues();

            $.ajax({
                url: globalConfig.path + '/register',
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
                    self.$btnRegister.removeClass('disabled');
                }
            });
        }
    };

    register.init();
});