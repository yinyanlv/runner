$(function () {

    var register = {
        init: function () {
            var self = this;

            self.initElements();
            self.initEvents();
        },

        initElements: function () {
            var self = this;

            self.$inputUsername = $('#username');
            self.$inputPassword = $('#password');
            self.$inputRepeatPassword = $('#repeat-password');
            self.$inputEmail = $('#email');
            self.$btnRegister = $('#btn-register');
            self.$needCheckInputs = $('.input-line input');
        },

        initEvents: function () {
            var self = this;

            self.$btnRegister.on('click', function () {

                self.register();
            });
        },

        register: function () {
            var self = this;

            if (!self.isValid()) return;

            var params = self.getParams();


            $.ajax({
                url: globalConfig.path + '/register',
                type: 'POST',
                dataType: 'application/json',
                data: params,
                success: function () {

                },
                error: function () {

                }
            });
        },

        getParams: function () {
            var self = this;

            return {
                username: $.trim($inputUsername.val()),
                password: $.trim($inputPassword.val()),
                email: $.trim($inputEmail.val())
            };
        },

        isValid: function() {
            var self = this;
            var isValid = true;

            for (var i = 0; i < self.$needCheckInputs.length; i++) {

                if (!self.checkInput(self.$needCheckInputs[i])) {

                    isValid = false;
                    break;
                }
            }

            return isValid;
        },

        checkInput: function (input) {
            var self = this;
            var $input = $(input);

        }
    };

    register.init();
});