pipeline {
    agent any

    triggers {
        githubPush()
    }

    environment {
        PATH = "/var/lib/jenkins/.cargo/bin:${env.PATH}"
    }

    stages {
        stage('Checkout') {
            steps { checkout scm }
        }
        stage('Check tools') {
            steps { sh 'cargo --version && rustc --version' }
        }
        stage('Test') {
            steps { sh 'cargo test' }
        }
        stage('Build') {
            steps { sh 'cargo build --release' }
        }
        stage('Archive') {
            steps {
                archiveArtifacts artifacts: 'target/release/hello_jenkins', fingerprint: true
            }
        }
    }

    post {
        success { echo 'Pipeline passed' }
        failure { echo 'Pipeline failed' }
    }
}
