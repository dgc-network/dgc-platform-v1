#!groovy

// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

pipeline {
    agent {
        node {
            label 'master'
            customWorkspace "workspace/${env.BUILD_TAG}"
        }
    }

    triggers {
        cron(env.BRANCH_NAME == 'master' ? 'H 2 * * *' : '')
    }

    options {
        timestamps()
        buildDiscarder(logRotator(daysToKeepStr: '31'))
    }

    environment {
        ISOLATION_ID = sh(returnStdout: true, script: 'printf $BUILD_TAG | sha256sum | cut -c1-64').trim()
        COMPOSE_PROJECT_NAME = sh(returnStdout: true, script: 'printf $BUILD_TAG | sha256sum | cut -c1-64').trim()
        JENKINS_UID = sh(returnStdout: true, script: "id -u ${USER}").trim()
    }

    stages {
        stage('Check Whitelist') {
            steps {
                readTrusted 'bin/whitelist'
                sh './bin/whitelist "$CHANGE_AUTHOR" /etc/jenkins-authorized-builders'
            }
            when {
                not {
                    branch 'master'
                }
            }
        }

        stage('Check for Signed-Off Commits') {
            steps {
                sh '''#!/bin/bash -l
                    if [ -v CHANGE_URL ] ;
                    then
                        temp_url="$(echo $CHANGE_URL |sed s#github.com/#api.github.com/repos/#)/commits"
                        pull_url="$(echo $temp_url |sed s#pull#pulls#)"

                        IFS=$'\n'
                        for m in $(curl -s "$pull_url" | grep "message") ; do
                            if echo "$m" | grep -qi signed-off-by:
                            then
                              continue
                            else
                              echo "FAIL: Missing Signed-Off Field"
                              echo "$m"
                              exit 1
                            fi
                        done
                        unset IFS;
                    fi
                '''
            }
        }

        stage("Build dgc-platform UI Test Dependencies") {
            steps {
                sh 'docker build dgc-platform-ui -f dgc-platform-ui/docker/test/Dockerfile -t dgc-platform-ui:$ISOLATION_ID'
                sh 'docker build dgc-platform-ui/saplings/product -f dgc-platform-ui/saplings/product/test/Dockerfile -t product-sapling:$ISOLATION_ID'
                sh 'docker build dgc-platform-ui/saplings/profile -f dgc-platform-ui/saplings/profile/test/Dockerfile -t profile-sapling:$ISOLATION_ID'
            }
        }

        stage("Run Lint on dgc-platform UI") {
            steps {
                sh 'docker run --rm --env CI=true dgc-platform-ui:$ISOLATION_ID yarn lint'
                sh 'docker run --rm --env CI=true product-sapling:$ISOLATION_ID yarn lint'
                sh 'docker run --rm --env CI=true profile-sapling:$ISOLATION_ID yarn lint'
            }
        }

        stage("Run dgc-platform UI tests") {
            steps {
                sh 'docker run --rm --env CI=true dgc-platform-ui:$ISOLATION_ID yarn test'
                sh 'docker run --rm --env CI=true product-sapling:$ISOLATION_ID yarn test'
            }
        }

        stage("Run Lint on dgc-platform") {
            steps {
                sh 'docker build . -f docker/lint -t lint-dgc-platform:$ISOLATION_ID'
                sh 'docker run --rm -v $(pwd):/project/dgc-platform lint-dgc-platform:$ISOLATION_ID'
            }
        }

        // Use a docker container to build and protogen, so that the Jenkins
        // environment doesn't need all the dependencies.
        stage("Build dgc-platform Test Dependencies") {
            steps {
                sh 'VERSION=AUTO_STRICT REPO_VERSION=$(./bin/get_version) docker-compose -f docker-compose.yaml build --force-rm'
                sh 'docker-compose -f docker/compose/grid_tests.yaml build --force-rm'
            }
        }

        stage("Run dgc-platform unit tests") {
            steps {
                sh 'docker-compose -f docker/compose/grid_tests.yaml up --abort-on-container-exit --exit-code-from grid_tests'
                sh 'docker-compose -f docker/compose/grid_tests.yaml down'
            }
        }

        stage("Run integration tests") {
            steps {
                sh './bin/run_integration_tests'
            }
        }

        stage("Build dgc-platform-daemon with experimental features") {
            steps {
                sh 'ISOLATION_ID=$ISOLATION_ID"experimental" CARGO_ARGS="-- --features experimental" docker-compose -f docker-compose.yaml build dgc-platform-daemon'
            }
        }

        stage("Create git archive") {
            steps {
                sh '''
                    REPO=$(git remote show -n origin | grep Fetch | awk -F'[/.]' '{print $6}')
                    VERSION=`git describe --dirty`
                    git archive HEAD --format=zip -9 --output=$REPO-$VERSION.zip
                    git archive HEAD --format=tgz -9 --output=$REPO-$VERSION.tgz
                '''
            }
        }

        stage ("Build documentation") {
            steps {
                sh 'docker build . -f docs/dgc-platform-build-docs -t dgc-platform-build-docs:$ISOLATION_ID'
                sh 'docker run --rm -v $(pwd):/project/dgc-platform dgc-platform-build-docs:$ISOLATION_ID'
            }
        }

        stage("Build artifacts") {
            steps {
                sh 'mkdir -p build/debs'
                sh 'docker run --rm -v $(pwd)/build/debs/:/debs dgc-platform-daemon:${ISOLATION_ID} bash -c "cp /tmp/dgc-platform*.deb /debs && chown -R ${JENKINS_UID} /debs"'
            }
        }
    }

    post {
        always {
            sh 'docker-compose -f docker/compose/grid_tests.yaml down'
        }
        success {
            archiveArtifacts '*.tgz, *.zip, build/debs/*.deb, docs/build/html/**, docs/build/latex/*.pdf'
        }
        aborted {
            error "Aborted, exiting now"
        }
        failure {
            error "Failed, exiting now"
        }
    }
}
