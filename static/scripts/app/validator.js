$(function () {

    function Validator(options) {

        var $form = $(options.form);
        var $needCheckInputs = $form.find('.input-line input');
        var $errorLine = $form.find('#error-line');
        var $errorMessage = $form.find('#error-message');
        var autoFocusErrorInput = true;

        $needCheckInputs.on({
            blur: function () {
                validator.isValid();
                autoFocusErrorInput = false;
            }
        });

        $needCheckInputs.on('keyup', function (e) {

            if (e.keyCode === 13) {

                autoFocusErrorInput = true;
                options.submit && options.submit();
            }
        });

        var validator = {

            isValid: function() {
                var self = this;
                var isValid = true;

                self.hideError();

                for (var i = 0; i < $needCheckInputs.length; i++) {

                    if (!self.checkInput($needCheckInputs[i])) {

                        isValid = false;
                        break;
                    }
                }

                return isValid;
            },

            checkInput: function (input) {
                var self = this;
                var validTasks = ['required', 'min-length', 'max-length', 'pattern', 'equal'];
                var $input = $(input);
                var isValid = true;

                for (var i = 0; i < validTasks.length; i++) {

                    var optsStr = $input.data(validTasks[i]);

                    if (!optsStr) continue;

                    var opts = JSON.parse(optsStr.replace(/\'/g, '\"'));  // 将字符串中的'，替换成"，否则，JSON.parse报错

                    switch (validTasks[i]) {
                        case 'required':

                            if ($.trim($input.val()).length === 0) {

                                self.showError($input, opts.message);

                                isValid = false;
                                break;
                            }

                            break;
                        case 'min-length':

                            if ($.trim($input.val()).length < parseInt(opts.value || 0)) {

                                self.showError($input, opts.message);

                                isValid = false;
                                break;
                            }

                            break;
                        case 'max-length':

                            if ($.trim($input.val()).length > parseInt(opts.value || 0)) {

                                self.showError($input, opts.message);

                                isValid = false;
                                break;
                            }

                            break;
                        case 'pattern':

                            var reg = new RegExp(opts.value);

                            if (!reg.test($.trim($input.val()))) {

                                self.showError($input, opts.message);

                                isValid = false;
                                break;
                            }

                            break;
                        case 'equal':

                            var $comparedInput = $form.find(opts.value);

                            if ($comparedInput.val() !== $input.val()) {

                                self.showError($input, opts.message);

                                isValid = false;
                                break;
                            }

                            break;
                        default:
                            break;
                    }
                }

                return isValid;
            },

            getValues: function () {
                var self = this;
                var values = {};

                for (var i = 0; i < $needCheckInputs.length; i++) {

                    var input = $needCheckInputs[i];

                    values[input.name] = $.trim(input.value);
                }

                return values;
            },

            showError: function ($input, message) {
                var self = this;

                $input && $input.addClass('error');
                $errorLine.show();
                $errorMessage.html(message);

                if (autoFocusErrorInput) {
                    autoFocusErrorInput = false;
                    $input && $input.focus();
                }
            },

            hideError: function () {
                var self = this;

                $needCheckInputs.removeClass('error');
                $errorLine.hide();
                $errorMessage.html('');
            }
        };

        return validator;
    }

    window.Validator = Validator;
});