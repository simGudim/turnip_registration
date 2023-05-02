pipeline {
  agent any
  tools { 
        maven 'Maven_3_5_2'  
    }

    stages {
        stage('Build') { 
                steps { 
                withDockerRegistry([credentialsId: "dockerlogin", url: ""]) {
                    script{
                    app =  docker.build("turnip/turnip_registration:1.0")
                    }
                }
                }
        }

        stage('Push') {
                steps {
                    script{
                        docker.withRegistry('https://676180064909.dkr.ecr.ca-central-1.amazonaws.com', 'ecr:ca-central-1:aws-credentials') {
                        app.push("latest")
                        }
                    }
                }
            }
    }
}
