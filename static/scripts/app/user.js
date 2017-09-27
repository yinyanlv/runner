$(function () {

    var user = {

        init: function () {
            var self = this;

            self.initElements();
            self.initPlugins();
            self.initEvents();
        },

        initElements: function () {
            var self = this;

            self.$btnSaveUser = $('#btn-save-user');
            self.$btnChangePassword = $('#btn-change-password');
        },

        initPlugins: function () {
            var self = this;

            self.userValidator = Validator ? new Validator({
                form: '#form-user',
                submit: self.changePassword.bind(this)
            }) : null;

            self.passwordValidator = Validator ? new Validator({
                form: '#form-change-password',
                submit: self.changePassword.bind(this)
            }) : null;
        },

        initEvents: function () {
            var self = this;

            self.$btnSaveUser.on('click', function () {

                self.saveUser();
            });

            self.$btnChangePassword.on('click', function () {

                self.changePassword();
            });
        },

        saveUser: function () {
            var self = this;

            if (!self.userValidator.isValid()) return;
            if (self.$btnSaveUser.is('disabled')) return;

            self.$btnSaveUser.addClass('disabled');

            var params = self.userValidator.getValues();

            $.ajax({
                url: globalConfig.path + '/user/update',
                type: 'POST',
                data: params,
                success: function (res) {

                    if (res.success) {

                       window.location.reload();
                    } else {

                        self.userValidator.showError(null, res.message);
                    }
                },
                complete: function () {
                    self.$btnSaveUser.removeClass('disabled');
                }
            });
        },

        changePassword: function () {
            var self = this;

            if (!self.passwordValidator.isValid()) return;
            if (self.$btnChangePassword.is('disabled')) return;

            self.$btnChangePassword.addClass('disabled');

            var params = self.passwordValidator.getValues();

            $.ajax({
                url: globalConfig.path + '/user/change-password',
                type: 'POST',
                data: params,
                success: function (res) {

                    if (res.success) {

                        window.location.href = res.data;
                    } else {

                        self.passwordValidator.showError(null, res.message);
                    }
                },
                complete: function () {
                    self.$btnChangePassword.removeClass('disabled');
                }
            });
        }
    };

    user.init();
});