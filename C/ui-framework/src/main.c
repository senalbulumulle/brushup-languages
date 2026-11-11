#include <GLFW/glfw3.h>

#include <leif/leif.h>

#include <stdio.h>


// @docs
// Sets the window size
static int win_w = 800;
static int win_h = 600;

static void framebuffer_size_callback(GLFWwindow * window, int width, int height) {
        (void) window;
        win_w = width;
        win_h = height;
        glViewport(0, 0, width, height);
        lf_resize_display(width, height);
}

int main(void) {
        if (!glfwInit()) {
                fprintf(stderr, "Failed to initialize GLFW\n");
                return 1;
        }

        glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
        glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
        glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
        #if defined(__APPLE__)
        glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GL_TRUE);
        #endif

        GLFWwindow * window = glfwCreateWindow(win_w, win_h, "Leif UI Framework", NULL, NULL);
        if (!window) {
                fprintf(stderr, "Failed to create GLFW window\n");
                glfwTerminate();
                return 1;
        }

        glfwMakeContextCurrent(window);
        glfwSetFramebufferSizeCallback(window, framebuffer_size_callback);

        lf_init_glfw(win_w, win_h, window);

        while (!glfwWindowShouldClose(window)) {
                glClearColor(0.10 f, 0.10 f, 0.10 f, 1.0 f);
                glClear(GL_COLOR_BUFFER_BIT);

                lf_begin();
                lf_text("Hello, Leif!");
                if (lf_button_fixed("Click me", 120, 36) == LF_CLICKED) {
                        printf("button pressed\n");
                }
                lf_end();

                glfwSwapBuffers(window);
                glfwPollEvents();
        }

        lf_terminate();
        glfwDestroyWindow(window);
        glfwTerminate();
        return 0;
}